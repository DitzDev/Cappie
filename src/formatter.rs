use crate::level::Level;
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Converts a log record as emitted by [`Logger`] into its **final textual form** that gets
/// written by an [`Output`].  The trait is intentionally minimal: implement the single
/// [`format`] method and you are good to go!
///
/// Custom formatters allow integrating with existing log pipelines (e.g. _ELK_, _Datadog_,
/// _FluentBit_) or bespoke CLI tooling.
///
/// # Parameters
/// * `level`     – severity of the message
/// * `msg`       – human‑readable log message
/// * `fields`    – structured key/value pairs attached to the event
/// * `timestamp` – wall‑clock time of the call (UTC)
/// * `name`      – hierarchical logger name (`frontend.http` etc.)
pub trait Formatter: Send + Sync {
    fn format(&self, level: Level, msg: &str, fields: &Map<String, Value>, timestamp: DateTime<Utc>, name: &str) -> String;
}

/// Serialises a record to **newline‑delimited JSON (ND‑JSON)** – perfectly suited for
/// machine ingestion.  All user‑supplied fields are flattened into the top‑level object so
/// they can be queried without additional object navigation.
///
/// Example output (pretty‑printed for readability):
/// ```jsonc
/// {
///   "level": 30,
///   "time": "2025-06-21T12:34:56Z",
///   "name": "backend.api",
///   "msg": "user created",
///   "user_id": 42,
///   "plan": "pro"
/// }
/// ```
pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, level: Level, msg: &str, fields: &Map<String, Value>, timestamp: DateTime<Utc>, name: &str) -> String {
        let mut log_entry = Map::new();
        
        log_entry.insert("level".to_string(), Value::Number(level.value().into()));
        log_entry.insert("time".to_string(), Value::String(timestamp.to_rfc3339()));
        log_entry.insert("name".to_string(), Value::String(name.to_string()));
        log_entry.insert("msg".to_string(), Value::String(msg.to_string()));
        
        for (k, v) in fields {
            log_entry.insert(k.clone(), v.clone());
        }
        
        serde_json::to_string(&log_entry).unwrap_or_default()
    }
}

/// Defines the position of different components in the log output
#[derive(Debug, Clone, PartialEq)]
#[derive(Hash)]
#[derive(Eq)]
pub enum ComponentPosition {
    /// Component appears before everything else
    Start,
    /// Component appears after timestamp
    AfterTime,
    /// Component appears after logger name
    AfterName,
    /// Component appears after level
    AfterLevel,
    /// Component appears after message
    AfterMessage,
    /// Component appears at the end
    End,
}

/// Represents a template token that can be positioned and styled
#[derive(Debug, Clone)]
pub struct TemplateComponent {
    /// The type of component
    pub component_type: ComponentType,
    /// Position in the output
    pub position: ComponentPosition,
    /// Optional color for this component
    pub color: Option<String>,
    /// Optional prefix (e.g., "[" for timestamp)
    pub prefix: Option<String>,
    /// Optional suffix (e.g., "]" for timestamp)
    pub suffix: Option<String>,
}

/// Types of components that can be included in log output
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentType {
    Timestamp,
    LoggerName,
    Level,
    Message,
    Fields,
    CustomText(String),
}

/// A highly flexible formatter that allows complete customization of log output format
///
/// Users can define exactly how they want their logs to appear by:
/// * Positioning components anywhere in the output
/// * Adding custom colors to any component
/// * Adding prefixes and suffixes (brackets, parentheses, etc.)
/// * Including custom text at any position
/// * Controlling timestamp format
///
/// # Examples
///
/// Default format: `[12:34:56] (logger) INFO: message key=value`
/// ```rust
/// let formatter = FlexibleFormatter::new();
/// ```
///
/// Custom format: `message INFO [12:34:56]: fields`
/// ```rust
/// let formatter = FlexibleFormatter::new()
///     .clear_components()
///     .add_component(ComponentType::Message, ComponentPosition::Start, None, None, None)
///     .add_component(ComponentType::Level, ComponentPosition::AfterMessage, Some("\x1b[31m".to_string()), Some(" ".to_string()), None)
///     .add_component(ComponentType::Timestamp, ComponentPosition::AfterLevel, None, Some(" [".to_string()), Some("]".to_string()))
///     .add_component(ComponentType::CustomText(": ".to_string()), ComponentPosition::AfterTime, None, None, None)
///     .add_component(ComponentType::Fields, ComponentPosition::End, None, None, None);
/// ```
pub struct FlexibleFormatter {
    pub time_format: String,
    pub reset_color: String,
    pub components: Vec<TemplateComponent>,
}

impl Default for FlexibleFormatter {
    fn default() -> Self {
        let mut components = Vec::new();
        
        // Default format: [HH:mm:SS] (name) LEVEL: message fields
        components.push(TemplateComponent {
            component_type: ComponentType::Timestamp,
            position: ComponentPosition::Start,
            color: None,
            prefix: Some("[".to_string()),
            suffix: Some("]".to_string()),
        });
        
        components.push(TemplateComponent {
            component_type: ComponentType::LoggerName,
            position: ComponentPosition::AfterTime,
            color: None,
            prefix: Some(" (".to_string()),
            suffix: Some(")".to_string()),
        });
        
        components.push(TemplateComponent {
            component_type: ComponentType::Level,
            position: ComponentPosition::AfterName,
            color: None,
            prefix: Some(" ".to_string()),
            suffix: None,
        });
        
        components.push(TemplateComponent {
            component_type: ComponentType::CustomText(":".to_string()),
            position: ComponentPosition::AfterLevel,
            color: None,
            prefix: None,
            suffix: None,
        });
        
        components.push(TemplateComponent {
            component_type: ComponentType::Message,
            position: ComponentPosition::AfterLevel,
            color: None,
            prefix: Some(" ".to_string()),
            suffix: None,
        });
        
        components.push(TemplateComponent {
            component_type: ComponentType::Fields,
            position: ComponentPosition::End,
            color: None,
            prefix: Some(" ".to_string()),
            suffix: None,
        });
        
        Self {
            time_format: "%H:%M:%S".to_string(),
            reset_color: "\x1b[0m".to_string(),
            components,
        }
    }
}

impl FlexibleFormatter {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the timestamp format
    pub fn with_time_format(mut self, format: &str) -> Self {
        self.time_format = format.to_string();
        self
    }
    
    /// Clear all components (start with empty formatter)
    pub fn clear_components(mut self) -> Self {
        self.components.clear();
        self
    }
    
    /// Add a component to the formatter
    pub fn add_component(
        mut self, 
        component_type: ComponentType, 
        position: ComponentPosition,
        color: Option<String>,
        prefix: Option<String>,
        suffix: Option<String>
    ) -> Self {
        self.components.push(TemplateComponent {
            component_type,
            position,
            color,
            prefix,
            suffix,
        });
        self
    }
    
    /// Add a timestamp component
    pub fn add_timestamp(self, position: ComponentPosition, color: Option<String>, prefix: Option<String>, suffix: Option<String>) -> Self {
        self.add_component(ComponentType::Timestamp, position, color, prefix, suffix)
    }
    
    /// Add a logger name component
    pub fn add_logger_name(self, position: ComponentPosition, color: Option<String>, prefix: Option<String>, suffix: Option<String>) -> Self {
        self.add_component(ComponentType::LoggerName, position, color, prefix, suffix)
    }
    
    /// Add a level component
    pub fn add_level(self, position: ComponentPosition, color: Option<String>, prefix: Option<String>, suffix: Option<String>) -> Self {
        self.add_component(ComponentType::Level, position, color, prefix, suffix)
    }
    
    /// Add a message component
    pub fn add_message(self, position: ComponentPosition, color: Option<String>, prefix: Option<String>, suffix: Option<String>) -> Self {
        self.add_component(ComponentType::Message, position, color, prefix, suffix)
    }
    
    /// Add a fields component
    pub fn add_fields(self, position: ComponentPosition, color: Option<String>, prefix: Option<String>, suffix: Option<String>) -> Self {
        self.add_component(ComponentType::Fields, position, color, prefix, suffix)
    }
    
    /// Add custom text
    pub fn add_custom_text(self, text: &str, position: ComponentPosition, color: Option<String>) -> Self {
        self.add_component(ComponentType::CustomText(text.to_string()), position, color, None, None)
    }
    
    /// Disable all colors
    pub fn with_no_colors(mut self) -> Self {
        for component in &mut self.components {
            component.color = None;
        }
        self.reset_color.clear();
        self
    }
}

impl Formatter for FlexibleFormatter {
    fn format(&self, level: Level, msg: &str, fields: &Map<String, Value>, timestamp: DateTime<Utc>, name: &str) -> String {
        let time_str = timestamp.format(&self.time_format).to_string();
        let level_str = level.as_str();
        let fields_str = if !fields.is_empty() {
            fields.iter()
                .map(|(k, v)| format!("{}={}", k, format_value(v)))
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            String::new()
        };
        
        // Group components by position
        let mut positioned_components: HashMap<ComponentPosition, Vec<&TemplateComponent>> = HashMap::new();
        for component in &self.components {
            positioned_components.entry(component.position.clone()).or_insert_with(Vec::new).push(component);
        }
        
        let mut result = String::new();
        
        // Process components in order
        let positions = [
            ComponentPosition::Start,
            ComponentPosition::AfterTime,
            ComponentPosition::AfterName,
            ComponentPosition::AfterLevel,
            ComponentPosition::AfterMessage,
            ComponentPosition::End,
        ];
        
        for position in &positions {
            if let Some(components) = positioned_components.get(position) {
                for component in components {
                    let content = match &component.component_type {
                        ComponentType::Timestamp => Some(time_str.as_str()),
                        ComponentType::LoggerName => Some(name),
                        ComponentType::Level => Some(level_str),
                        ComponentType::Message => Some(msg),
                        ComponentType::Fields => if !fields_str.is_empty() { Some(fields_str.as_str()) } else { None },
                        ComponentType::CustomText(text) => Some(text.as_str()),
                    };
                    
                    if let Some(content) = content {
                        // Add prefix
                        if let Some(ref prefix) = component.prefix {
                            result.push_str(prefix);
                        }
                        
                        // Add color
                        if let Some(ref color) = component.color {
                            result.push_str(color);
                        }
                        
                        // Add content
                        result.push_str(content);
                        
                        // Add reset color
                        if component.color.is_some() && !self.reset_color.is_empty() {
                            result.push_str(&self.reset_color);
                        }
                        
                        // Add suffix
                        if let Some(ref suffix) = component.suffix {
                            result.push_str(suffix);
                        }
                    }
                }
            }
        }
        
        result
    }
}

/// Human‑friendly single‑line layout inspired by `env_logger`.
///
/// * **Timestamp** – formatted according to [`time_format`](Self::time_format) (default:
///   `%H:%M:%S`).
/// * **Logger name** – in parentheses.
/// * **Level** – colourised if the respective ANSI escape code is configured in
///   [`colors`](Self::colors).
/// * **Message**.
/// * **Fields** – appended as `key=value` pairs.
///
/// # Example
/// ```text
/// [12:34:56] (auth) INFO: login succeeded user=42
/// ```
pub struct PrettyFormatter {
    pub time_format: String,
    pub colors: HashMap<Level, String>,
    pub reset_color: String,
}

impl Default for PrettyFormatter {
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert(Level::Trace, "\x1b[90m".to_string()); // Bright Black
        colors.insert(Level::Debug, "\x1b[36m".to_string()); // Cyan
        colors.insert(Level::Info, "\x1b[32m".to_string());  // Green
        colors.insert(Level::Warn, "\x1b[33m".to_string());  // Yellow
        colors.insert(Level::Error, "\x1b[31m".to_string()); // Red
        colors.insert(Level::Fatal, "\x1b[35m".to_string()); // Magenta
        
        Self {
            time_format: "%H:%M:%S".to_string(),
            colors,
            reset_color: "\x1b[0m".to_string(),
        }
    }
}

impl PrettyFormatter {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_time_format(mut self, format: &str) -> Self {
        self.time_format = format.to_string();
        self
    }
    
    pub fn with_color(mut self, level: Level, color: &str) -> Self {
        self.colors.insert(level, color.to_string());
        self
    }
    
    pub fn with_no_colors(mut self) -> Self {
        self.colors.clear();
        self.reset_color.clear();
        self
    }
}

impl Formatter for PrettyFormatter {
    fn format(&self, level: Level, msg: &str, fields: &Map<String, Value>, timestamp: DateTime<Utc>, name: &str) -> String {
        let time_str = timestamp.format(&self.time_format).to_string();
        let level_str = level.as_str();
        
        let color = self.colors.get(&level).cloned().unwrap_or_default();
        let reset = &self.reset_color;
        
        let mut result = format!("[{}] ({}) {}{}{}: {}", 
            time_str, name, color, level_str, reset, msg);
        
        if !fields.is_empty() {
            let fields_str = fields.iter()
                .map(|(k, v)| format!("{}={}", k, format_value(v)))
                .collect::<Vec<_>>()
                .join(" ");
            result.push_str(&format!(" {}", fields_str));
        }
        
        result
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(_) | Value::Object(_) => serde_json::to_string(value).unwrap_or_default(),
    }
}