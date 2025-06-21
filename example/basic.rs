use cappie::{Logger, Level, PrettyFormatter, JsonFormatter};

fn main() {
    // Basic JSON logging
    let logger = Logger::new("my-app")
        .with_level(Level::Debug);
    
    logger.info("Application started");
    logger.debug("Debug information");
    logger.warn("This is a warning");
    logger.error("An error occurred");
    
    // Logging with additional fields
    logger.info_with("User logged in", |log| {
        log.string("user_id", "12345")
           .string("ip", "192.168.1.1")
           .bool("first_time", true);
    });
    
    println!("\n--- Pretty Logging ---\n");

    // Pretty logging
    let pretty_logger = Logger::new("my-app")
        .with_formatter(Box::new(PrettyFormatter::new()))
        .with_level(Level::Trace);
    
    pretty_logger.trace("Trace message");
    pretty_logger.debug("Debug message");
    pretty_logger.info("Info message");
    pretty_logger.warn("Warning message");
    pretty_logger.error("Error message");
    pretty_logger.fatal("Fatal message");
    
    // Pretty logging with fields
    pretty_logger.error_with("Database connection failed", |log| {
        log.string("host", "localhost")
           .number("port", 5432)
           .string("database", "myapp");
    });
    
    println!("\n--- Custom Colors ---\n");
    
    // Custom colors
    let custom_logger = Logger::new("my-app")
        .with_formatter(Box::new(
            PrettyFormatter::new()
                .with_color(Level::Error, "\x1b[91m") // Bright red
                .with_color(Level::Warn, "\x1b[93m")  // Bright yellow
                .with_time_format("%Y-%m-%d %H:%M:%S")
        ));
    
    custom_logger.warn("Custom colored warning");
    custom_logger.error("Custom colored error");
    
    // Child logger
    let child = logger.child("auth");
    child.info("Authentication module initialized");
}
