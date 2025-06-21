use crate::level::Level;
use crate::formatter::{Formatter, JsonFormatter, PrettyFormatter};
use crate::output::{Output, StdoutOutput};
use chrono::Utc;
use serde_json::{Map, Value};

/// Main façade that **users interact with**.  A logger is cheap to clone because it only
/// contains a couple of `Arc`s/`Box`es, so feel free to pass it around.
///
/// The struct is designed for **builder‑style configuration**:
///
/// ```
/// use simple_logger::{Logger, Level, PrettyFormatter};
///
/// let log = Logger::new("backend")
///     .with_level(Level::Debug)
///     .with_formatter(Box::new(PrettyFormatter::new()))
///     .with_output(Box::new(simple_logger::output::StderrOutput))
///     .with_field("version", env!("CARGO_PKG_VERSION"));
/// ```
///
/// **Thread‑safety:** all methods take `&self`; shared state is protected by `Arc`.  You can
/// therefore use the same instance from multiple threads without additional locking.
pub struct Logger {
    name: String,
    level: Level,
    formatter: Box<dyn Formatter>,
    output: Box<dyn Output>,
    base_fields: Map<String, Value>,
}

impl Logger {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            level: Level::Info,
            formatter: Box::new(JsonFormatter),
            output: Box::new(StdoutOutput),
            base_fields: Map::new(),
        }
    }
    
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
    
    pub fn with_formatter(mut self, formatter: Box<dyn Formatter>) -> Self {
        self.formatter = formatter;
        self
    }
    
    pub fn with_output(mut self, output: Box<dyn Output>) -> Self {
        self.output = output;
        self
    }
    
    pub fn with_field<T: Into<Value>>(mut self, key: &str, value: T) -> Self {
        self.base_fields.insert(key.to_string(), value.into());
        self
    }
    
    pub fn pretty() -> Self {
        Self::new("app").with_formatter(Box::new(PrettyFormatter::new()))
    }
    
    pub fn child(&self, name: &str) -> Self {
        let child_name = if self.name.is_empty() {
            name.to_string()
        } else {
            format!("{}.{}", self.name, name)
        };
        
        Self {
            name: child_name,
            level: self.level,
            formatter: Box::new(JsonFormatter), // Reset to default for simplicity
            output: Box::new(StdoutOutput), // Reset to default for simplicity
            base_fields: self.base_fields.clone(),
        }
    }
    
    fn should_log(&self, level: Level) -> bool {
        level >= self.level
    }
    
    fn log(&self, level: Level, msg: &str, fields: Option<Map<String, Value>>) {
        if !self.should_log(level) {
            return;
        }
        
        let mut combined_fields = self.base_fields.clone();
        if let Some(fields) = fields {
            for (k, v) in fields {
                combined_fields.insert(k, v);
            }
        }
        
        let timestamp = Utc::now();
        let formatted = self.formatter.format(level, msg, &combined_fields, timestamp, &self.name);
        self.output.write(&formatted);
    }
    
    pub fn trace(&self, msg: &str) {
        self.log(Level::Trace, msg, None);
    }
    
    pub fn trace_with<F>(&self, msg: &str, f: F) 
    where
        F: FnOnce(&mut LogBuilder),
    {
        let mut builder = LogBuilder::new();
        f(&mut builder);
        self.log(Level::Trace, msg, Some(builder.fields));
    }
    
    pub fn debug(&self, msg: &str) {
        self.log(Level::Debug, msg, None);
    }
    
    pub fn debug_with<F>(&self, msg: &str, f: F) 
    where
        F: FnOnce(&mut LogBuilder),
    {
        let mut builder = LogBuilder::new();
        f(&mut builder);
        self.log(Level::Debug, msg, Some(builder.fields));
    }
    
    pub fn info(&self, msg: &str) {
        self.log(Level::Info, msg, None);
    }
    
    pub fn info_with<F>(&self, msg: &str, f: F) 
    where
        F: FnOnce(&mut LogBuilder),
    {
        let mut builder = LogBuilder::new();
        f(&mut builder);
        self.log(Level::Info, msg, Some(builder.fields));
    }
    
    pub fn warn(&self, msg: &str) {
        self.log(Level::Warn, msg, None);
    }
    
    pub fn warn_with<F>(&self, msg: &str, f: F) 
    where
        F: FnOnce(&mut LogBuilder),
    {
        let mut builder = LogBuilder::new();
        f(&mut builder);
        self.log(Level::Warn, msg, Some(builder.fields));
    }
    
    pub fn error(&self, msg: &str) {
        self.log(Level::Error, msg, None);
    }
    
    pub fn error_with<F>(&self, msg: &str, f: F) 
    where
        F: FnOnce(&mut LogBuilder),
    {
        let mut builder = LogBuilder::new();
        f(&mut builder);
        self.log(Level::Error, msg, Some(builder.fields));
    }
    
    pub fn fatal(&self, msg: &str) {
        self.log(Level::Fatal, msg, None);
    }
    
    pub fn fatal_with<F>(&self, msg: &str, f: F) 
    where
        F: FnOnce(&mut LogBuilder),
    {
        let mut builder = LogBuilder::new();
        f(&mut builder);
        self.log(Level::Fatal, msg, Some(builder.fields));
    }
}

pub struct LogBuilder {
    fields: Map<String, Value>,
}

impl LogBuilder {
    pub fn new() -> Self {
        Self {
            fields: Map::new(),
        }
    }
    
    pub fn field<T: Into<Value>>(&mut self, key: &str, value: T) -> &mut Self {
        self.fields.insert(key.to_string(), value.into());
        self
    }
    
    pub fn string(&mut self, key: &str, value: &str) -> &mut Self {
        self.fields.insert(key.to_string(), Value::String(value.to_string()));
        self
    }
    
    pub fn number<T: Into<serde_json::Number>>(&mut self, key: &str, value: T) -> &mut Self {
        self.fields.insert(key.to_string(), Value::Number(value.into()));
        self
    }
    
    pub fn bool(&mut self, key: &str, value: bool) -> &mut Self {
        self.fields.insert(key.to_string(), Value::Bool(value));
        self
    }
}