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