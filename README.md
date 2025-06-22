# Cappie

[![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/cappie.svg
[crates.io]: https://crates.io/crates/cappie

A fast, flexible JSON logger for Rust, inspired by Pino from Node.js.

## Features

- 🚀 **Fast & Simple** - High-performance logger with minimal overhead
- 📝 **JSON Logging** - Structured JSON output by default
- 🎨 **Pretty Printing** - Human-readable colorized output
- 🔧 **Highly Configurable** - Customize colors, formats, and outputs
- ✨ **Flexible Formatting** - Complete control over log output format and positioning
- 📊 **Structured Logging** - Add custom fields to log entries
- 👶 **Child Loggers** - Create contextual child loggers
- 📁 **Multiple Outputs** - Log to console, files, or multiple destinations
- 🌈 **Custom Colors** - Full control over ANSI color codes for any component

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cappie = "0.1.0"
```

Or install with `cargo` for new version:
```bash
cargo add cappie
```

## Quick Start

### JSON Logging (Default)

```rust
use cappie::{Logger, Level};

fn main() {
    let logger = Logger::new("my-app")
        .with_level(Level::Debug);
    
    logger.info("Application started");
    logger.error("Something went wrong");
    
    // With additional fields
    logger.info_with("User logged in", |log| {
        log.string("user_id", "12345")
           .string("ip", "192.168.1.1")
           .bool("first_time", true);
    });
}
```

**Output:**
```json
{"level":30,"time":"2024-01-15T10:30:45.123Z","name":"my-app","msg":"Application started"}
{"level":50,"time":"2024-01-15T10:30:45.124Z","name":"my-app","msg":"Something went wrong"}
{"level":30,"time":"2024-01-15T10:30:45.125Z","name":"my-app","msg":"User logged in","user_id":"12345","ip":"192.168.1.1","first_time":true}
```

### Pretty Logging

```rust
use cappie::{Logger, PrettyFormatter};

fn main() {
    let logger = Logger::new("my-app")
        .with_formatter(Box::new(PrettyFormatter::new()));
    
    logger.info("Server started");
    logger.warn("Memory usage high");
    logger.error("Database connection failed");
}
```

**Output:**
```
[10:30:45] (my-app) INFO: Server started
[10:30:45] (my-app) WARN: Memory usage high  
[10:30:45] (my-app) ERROR: Database connection failed
```

## 🌟 Flexible Formatting (NEW!)

The `FlexibleFormatter` gives you complete control over log output format. You can position any component anywhere, add custom colors, prefixes, suffixes, and even custom text.

### Basic Flexible Usage

```rust
use cappie::{Logger, FlexibleFormatter, ComponentType, ComponentPosition};

// Default format: [HH:mm:SS] (logger) LEVEL: message fields
let logger = Logger::new("app")
    .with_formatter(Box::new(FlexibleFormatter::new()));
```

### Custom Positioning Examples

#### 1. Message First Format
```rust
// Output: message LEVEL [HH:mm:SS]: fields
let logger = Logger::new("api")
    .with_formatter(Box::new(
        FlexibleFormatter::new()
            .clear_components()
            .add_message(ComponentPosition::Start, None, None, None)
            .add_level(ComponentPosition::AfterMessage, Some("\x1b[31m".to_string()), Some(" ".to_string()), None)
            .add_timestamp(ComponentPosition::AfterLevel, None, Some(" [".to_string()), Some("]".to_string()))
            .add_custom_text(": ", ComponentPosition::AfterTime, None)
            .add_fields(ComponentPosition::End, None, None, None)
    ));

logger.error("Database connection failed");
// Output: Database connection failed ERROR [10:30:45]: 
```

#### 2. Minimal Format
```rust
// Output: [LEVEL] message
let logger = Logger::new("minimal")
    .with_formatter(Box::new(
        FlexibleFormatter::new()
            .clear_components()
            .add_level(ComponentPosition::Start, Some("\x1b[33m".to_string()), Some("[".to_string()), Some("]".to_string()))
            .add_message(ComponentPosition::AfterLevel, None, Some(" ".to_string()), None)
    ));

logger.warn("This is a warning");
// Output: [WARN] This is a warning
```

#### 3. Time at the End
```rust
// Output: [logger] LEVEL: message | fields (at HH:mm:SS)
let logger = Logger::new("backend")
    .with_formatter(Box::new(
        FlexibleFormatter::new()
            .clear_components()
            .add_logger_name(ComponentPosition::Start, Some("\x1b[36m".to_string()), Some("[".to_string()), Some("]".to_string()))
            .add_level(ComponentPosition::AfterName, Some("\x1b[32m".to_string()), Some(" ".to_string()), None)
            .add_custom_text(": ", ComponentPosition::AfterLevel, None)
            .add_message(ComponentPosition::AfterLevel, None, None, None)
            .add_fields(ComponentPosition::AfterMessage, None, Some(" | ".to_string()), None)
            .add_timestamp(ComponentPosition::End, Some("\x1b[90m".to_string()), Some(" (at ".to_string()), Some(")".to_string()))
    ));

logger.info_with("Server started", |log| { log.number("port", 8080); });
// Output: [backend] INFO: Server started | port=8080 (at 10:30:45)
```

#### 4. Highly Colorful Format
```rust
// Output: 🚀 [HH:mm:SS] {logger} <LEVEL> → message 📊 fields
let logger = Logger::new("colorful")
    .with_formatter(Box::new(
        FlexibleFormatter::new()
            .clear_components()
            .add_custom_text("🚀 ", ComponentPosition::Start, Some("\x1b[95m".to_string()))
            .add_timestamp(ComponentPosition::Start, Some("\x1b[94m".to_string()), Some("[".to_string()), Some("]".to_string()))
            .add_logger_name(ComponentPosition::AfterTime, Some("\x1b[96m".to_string()), Some(" {".to_string()), Some("}".to_string()))
            .add_level(ComponentPosition::AfterName, Some("\x1b[93m".to_string()), Some(" <".to_string()), Some(">".to_string()))
            .add_custom_text(" → ", ComponentPosition::AfterLevel, Some("\x1b[97m".to_string()))
            .add_message(ComponentPosition::AfterLevel, Some("\x1b[92m".to_string()), None, None)
            .add_fields(ComponentPosition::End, Some("\x1b[91m".to_string()), Some(" 📊 ".to_string()), None)
    ));

logger.info_with("App started", |log| { log.string("version", "1.0.0"); });
// Output: 🚀 [10:30:45] {colorful} <INFO> → App started 📊 version=1.0.0
```

### Component Types

The `FlexibleFormatter` supports these component types:

- **`ComponentType::Timestamp`** - The log timestamp
- **`ComponentType::LoggerName`** - The logger name  
- **`ComponentType::Level`** - The log level (INFO, ERROR, etc.)
- **`ComponentType::Message`** - The log message
- **`ComponentType::Fields`** - Additional structured fields
- **`ComponentType::CustomText(String)`** - Custom static text

### Component Positions

You can position components at these locations:

- **`ComponentPosition::Start`** - At the very beginning
- **`ComponentPosition::AfterTime`** - After the timestamp
- **`ComponentPosition::AfterName`** - After the logger name
- **`ComponentPosition::AfterLevel`** - After the log level
- **`ComponentPosition::AfterMessage`** - After the message
- **`ComponentPosition::End`** - At the very end

### FlexibleFormatter Methods

```rust
FlexibleFormatter::new()
    // Clear all default components
    .clear_components()
    
    // Add components with full control
    .add_component(ComponentType::Timestamp, ComponentPosition::Start, color, prefix, suffix)
    
    // Convenience methods for common components
    .add_timestamp(position, color, prefix, suffix)
    .add_logger_name(position, color, prefix, suffix)
    .add_level(position, color, prefix, suffix)
    .add_message(position, color, prefix, suffix)
    .add_fields(position, color, prefix, suffix)
    .add_custom_text("text", position, color)
    
    // Configuration
    .with_time_format("%H:%M:%S")
    .with_no_colors()
```

## Custom Colors & Traditional Pretty Format

```rust
use cappie::{Logger, PrettyFormatter, Level};

fn main() {
    let logger = Logger::new("my-app")
        .with_formatter(Box::new(
            PrettyFormatter::new()
                .with_color(Level::Error, "\x1b[91m") // Bright red
                .with_color(Level::Warn, "\x1b[93m")  // Bright yellow
                .with_time_format("%Y-%m-%d %H:%M:%S")
        ));
    
    logger.warn("Custom colored warning");
    logger.error("Custom colored error");
}
```

## Log Levels

| Level | Value | Description |
|-------|-------|-------------|
| `TRACE` | 10 | Very detailed information |
| `DEBUG` | 20 | Debug information |
| `INFO` | 30 | General information |
| `WARN` | 40 | Warning messages |
| `ERROR` | 50 | Error messages |
| `FATAL` | 60 | Fatal errors |

## Advanced Usage

### Child Loggers

Create contextual loggers that inherit parent configuration:

```rust
let parent = Logger::new("app");
let auth_logger = parent.child("auth");
let db_logger = parent.child("database");

auth_logger.info("User authenticated");  // Outputs: app.auth
db_logger.error("Connection timeout");   // Outputs: app.database
```

### Multiple Outputs

Log to console and file simultaneously:

```rust
use cappie::{Logger, FileOutput, MultiOutput, StdoutOutput};

let logger = Logger::new("my-app")
    .with_output(Box::new(
        MultiOutput::new()
            .add_output(Box::new(StdoutOutput))
            .add_output(Box::new(FileOutput::new("app.log")))
    ));

logger.info("This goes to both console and file");
```

### Base Fields

Add fields that appear in every log entry:

```rust
let logger = Logger::new("my-app")
    .with_field("version", "1.0.0")
    .with_field("env", "production")
    .with_field("service", "api-server");

logger.info("App started"); // Includes version, env, and service fields
```

### Structured Logging

Add contextual information to specific log entries:

```rust
logger.error_with("Database query failed", |log| {
    log.string("query", "SELECT * FROM users")
       .number("duration_ms", 1500)
       .string("table", "users")
       .bool("timeout", true);
});
```

## ANSI Color Codes

Cappie supports full ANSI color customization:

```rust
use cappie::{Logger, FlexibleFormatter, ComponentPosition, ComponentType};

let logger = Logger::new("colorful-app")
    .with_formatter(Box::new(
        FlexibleFormatter::new()
            .clear_components()
            .add_timestamp(ComponentPosition::Start, Some("\x1b[32m".to_string()), Some("[".to_string()), Some("]".to_string()))    // Green
            .add_level(ComponentPosition::AfterTime, Some("\x1b[33m".to_string()), Some(" ".to_string()), None)    // Yellow
            .add_message(ComponentPosition::AfterLevel, Some("\x1b[31m".to_string()), Some(": ".to_string()), None)   // Red
    ));
```

### Common ANSI Colors

| Color | Code | Color | Code |
|-------|------|-------|------|
| Black | `\x1b[30m` | Bright Black | `\x1b[90m` |
| Red | `\x1b[31m` | Bright Red | `\x1b[91m` |
| Green | `\x1b[32m` | Bright Green | `\x1b[92m` |
| Yellow | `\x1b[33m` | Bright Yellow | `\x1b[93m` |
| Blue | `\x1b[34m` | Bright Blue | `\x1b[94m` |
| Magenta | `\x1b[35m` | Bright Magenta | `\x1b[95m` |
| Cyan | `\x1b[36m` | Bright Cyan | `\x1b[96m` |
| White | `\x1b[37m` | Bright White | `\x1b[97m` |

## Examples

Run the included examples:

```bash
# Basic JSON and pretty logging
cargo run --example basic

# Pretty logging with colors and fields
cargo run --example pretty

# Custom colors, outputs, and configurations
cargo run --example custom

# Flexible formatter examples with various layouts
cargo run --example flexible
```

## API Reference

### Logger Methods

```rust
// Create a new logger
Logger::new("app-name")

// Configuration
.with_level(Level::Debug)
.with_formatter(Box::new(FlexibleFormatter::new()))
.with_output(Box::new(StdoutOutput))
.with_field("key", "value")

// Logging methods
.trace("message")
.debug("message")
.info("message")
.warn("message")
.error("message")
.fatal("message")

// Logging with fields
.info_with("message", |log| {
    log.string("key", "value")
       .number("count", 42)
       .bool("success", true);
})

// Create child logger
.child("module-name")
```

### FlexibleFormatter Methods

```rust
FlexibleFormatter::new()
    .with_time_format("%H:%M:%S")
    .clear_components()
    
    // Add specific components
    .add_timestamp(position, color, prefix, suffix)
    .add_logger_name(position, color, prefix, suffix)
    .add_level(position, color, prefix, suffix)
    .add_message(position, color, prefix, suffix)
    .add_fields(position, color, prefix, suffix)
    .add_custom_text("text", position, color)
    
    // Generic component addition
    .add_component(component_type, position, color, prefix, suffix)
    
    .with_no_colors()
```

### PrettyFormatter Methods

```rust
PrettyFormatter::new()
    .with_time_format("%H:%M:%S")
    .with_color(Level::Error, "\x1b[91m")
    .with_no_colors()
```

## Formatter Comparison

| Feature | JsonFormatter | PrettyFormatter | FlexibleFormatter |
|---------|---------------|-----------------|-------------------|
| JSON Output | ✅ | ❌ | ❌ |
| Human Readable | ❌ | ✅ | ✅ |
| Custom Colors | ❌ | ✅ | ✅ |
| Custom Positioning | ❌ | ❌ | ✅ |
| Custom Text | ❌ | ❌ | ✅ |
| Prefixes/Suffixes | ❌ | ❌ | ✅ |
| Performance | Fastest | Fast | Fast |
| Use Case | Machine parsing | Development | Custom layouts |

## Performance

Cappie is designed for high performance:

- **Minimal allocations** - Efficient memory usage
- **Fast JSON serialization** - Uses serde_json for speed
- **Lazy evaluation** - Fields only processed when logging level is enabled
- **Zero-cost abstractions** - No runtime overhead for disabled log levels
- **Flexible without overhead** - FlexibleFormatter adds minimal cost

## Real-World Usage Examples

### Web Server Logging
```rust
// API request logging with flexible format
let api_logger = Logger::new("api")
    .with_formatter(Box::new(
        FlexibleFormatter::new()
            .clear_components()
            .add_custom_text("🌐 ", ComponentPosition::Start, Some("\x1b[94m".to_string()))
            .add_timestamp(ComponentPosition::Start, Some("\x1b[90m".to_string()), None, None)
            .add_level(ComponentPosition::AfterTime, Some("\x1b[32m".to_string()), Some(" [".to_string()), Some("]".to_string()))
            .add_message(ComponentPosition::AfterLevel, None, Some(" ".to_string()), None)
            .add_fields(ComponentPosition::End, Some("\x1b[36m".to_string()), Some(" → ".to_string()), None)
    ));

api_logger.info_with("Request processed", |log| {
    log.string("method", "GET")
       .string("path", "/api/users")
       .number("status", 200)
       .number("duration_ms", 45);
});
```

### Database Operations
```rust
// Database logging with custom format
let db_logger = Logger::new("database")
    .with_formatter(Box::new(
        FlexibleFormatter::new()
            .clear_components()
            .add_custom_text("💾 DB ", ComponentPosition::Start, Some("\x1b[95m".to_string()))
            .add_level(ComponentPosition::Start, Some("\x1b[91m".to_string()), Some("[".to_string()), Some("]".to_string()))
            .add_message(ComponentPosition::AfterLevel, None, Some(" ".to_string()), None)
            .add_timestamp(ComponentPosition::End, Some("\x1b[90m".to_string()), Some(" @".to_string()), None)
            .add_fields(ComponentPosition::AfterMessage, Some("\x1b[33m".to_string()), Some(" {".to_string()), Some("}".to_string()))
    ));
```

## Comparison with Pino

| Feature | Cappie | Pino |
|---------|---------|------|
| JSON Logging | ✅ | ✅ |
| Pretty Printing | ✅ | ✅ |
| Flexible Formatting | ✅ | ❌ |
| Child Loggers | ✅ | ✅ |
| Custom Formatters | ✅ | ✅ |
| Multiple Outputs | ✅ | Limited |
| Custom Colors | ✅ | Limited |
| Custom Positioning | ✅ | ❌ |
| Performance | Very Fast | Fast |
| Language | Rust | JavaScript |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by [Pino](https://github.com/pinojs/pino) - The fast JSON logger for Node.js
- Built with [serde](https://serde.rs/) for serialization
- Uses [chrono](https://github.com/chronotope/chrono) for timestamp handling