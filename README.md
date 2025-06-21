# Cappie

A fast JSON logger for Rust, inspired by Pino from Node.js.

## Features

- 🚀 **Fast & Simple** - High-performance logger with minimal overhead
- 📝 **JSON Logging** - Structured JSON output by default
- 🎨 **Pretty Printing** - Human-readable colorized output
- 🔧 **Highly Configurable** - Customize colors, formats, and outputs
- 📊 **Structured Logging** - Add custom fields to log entries
- 👶 **Child Loggers** - Create contextual child loggers
- 📁 **Multiple Outputs** - Log to console, files, or multiple destinations
- 🎨 **Custom Colors** - Full control over ANSI color codes

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cappie = "0.1.0"
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

### Custom Colors & Format

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
use cappie::{Logger, PrettyFormatter, Level};

let logger = Logger::new("colorful-app")
    .with_formatter(Box::new(
        PrettyFormatter::new()
            .with_color(Level::Info, "\x1b[32m")    // Green
            .with_color(Level::Warn, "\x1b[33m")    // Yellow
            .with_color(Level::Error, "\x1b[31m")   // Red
            .with_color(Level::Debug, "\x1b[36m")   // Cyan
            .with_color(Level::Trace, "\x1b[90m")   // Dark gray
            .with_color(Level::Fatal, "\x1b[35m")   // Magenta
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
```

## API Reference

### Logger Methods

```rust
// Create a new logger
Logger::new("app-name")

// Configuration
.with_level(Level::Debug)
.with_formatter(Box::new(PrettyFormatter::new()))
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

### PrettyFormatter Methods

```rust
PrettyFormatter::new()
    .with_time_format("%H:%M:%S")
    .with_color(Level::Error, "\x1b[91m")
    .with_no_colors()
```

## Performance

Cappie is designed for high performance:

- **Minimal allocations** - Efficient memory usage
- **Fast JSON serialization** - Uses serde_json for speed
- **Lazy evaluation** - Fields only processed when logging level is enabled
- **Zero-cost abstractions** - No runtime overhead for disabled log levels

## Comparison with Pino

| Feature | Cappie | Pino |
|---------|---------|------|
| JSON Logging | ✅ | ✅ |
| Pretty Printing | ✅ | ✅ |
| Child Loggers | ✅ | ✅ |
| Custom Formatters | ✅ | ✅ |
| Multiple Outputs | ✅ | Limited |
| Custom Colors | ✅ | Limited |
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