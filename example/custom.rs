use cappie::{Logger, Level, PrettyFormatter, FileOutput, MultiOutput, StdoutOutput};

fn main() {
    println!("=== Custom Colors Demo ===\n");
    
    // Custom colors and time format
    let custom_logger = Logger::new("custom-app")
        .with_formatter(Box::new(
            PrettyFormatter::new()
                .with_color(Level::Trace, "\x1b[90m")  // Dark gray
                .with_color(Level::Debug, "\x1b[96m")  // Bright cyan
                .with_color(Level::Info, "\x1b[92m")   // Bright green
                .with_color(Level::Warn, "\x1b[93m")   // Bright yellow
                .with_color(Level::Error, "\x1b[91m")  // Bright red
                .with_color(Level::Fatal, "\x1b[95m")  // Bright magenta
                .with_time_format("%Y-%m-%d %H:%M:%S")
        ));
    
    custom_logger.trace("Custom trace with dark gray");
    custom_logger.debug("Custom debug with bright cyan");
    custom_logger.info("Custom info with bright green");
    custom_logger.warn("Custom warning with bright yellow");
    custom_logger.error("Custom error with bright red");
    custom_logger.fatal("Custom fatal with bright magenta");
    
    println!("\n=== Base Fields Demo ===\n");
    
    // Logger with base fields
    let app_logger = Logger::new("api-server")
        .with_formatter(Box::new(PrettyFormatter::new()))
        .with_field("version", "1.2.3")
        .with_field("environment", "production")
        .with_field("service", "auth-api");
    
    app_logger.info("Server started successfully");
    app_logger.warn("High memory usage detected");
    
    println!("\n=== Multiple Output Demo ===\n");
    
    // Multiple outputs (stdout + file)
    let multi_logger = Logger::new("multi-app")
        .with_formatter(Box::new(PrettyFormatter::new()))
        .with_output(Box::new(
            MultiOutput::new()
                .add_output(Box::new(StdoutOutput))
                .add_output(Box::new(FileOutput::new("app.log")))
        ));
    
    multi_logger.info("This message goes to both console and file");
    multi_logger.error("Error logged to both destinations");
    
    println!("\n=== Custom Time Format Demo ===\n");
    
    // Different time formats
    let time_logger = Logger::new("time-app")
        .with_formatter(Box::new(
            PrettyFormatter::new()
                .with_time_format("%I:%M:%S %p") // 12-hour format
        ));
    
    time_logger.info("Message with 12-hour time format");
    
    let iso_logger = Logger::new("iso-app")
        .with_formatter(Box::new(
            PrettyFormatter::new()
                .with_time_format("%Y-%m-%dT%H:%M:%S%.3fZ") // ISO format
        ));
    
    iso_logger.info("Message with ISO time format");
    
    println!("\nCheck 'app.log' file for logged messages!");
}