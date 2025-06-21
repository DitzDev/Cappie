use cappie::{Logger, Level, PrettyFormatter};

fn main() {
    // Pretty logging with default colors
    let logger = Logger::new("my-app")
        .with_formatter(Box::new(PrettyFormatter::new()))
        .with_level(Level::Trace);
    
    println!("=== Pretty Logging Demo ===\n");
    
    logger.trace("This is a trace message");
    logger.debug("Debug information for developers");
    logger.info("Application is running normally");
    logger.warn("This is a warning message");
    logger.error("An error occurred!");
    logger.fatal("Fatal error - application will exit");
    
    println!("\n=== With Additional Fields ===\n");
    
    // Logging with additional fields
    logger.info_with("User authentication successful", |log| {
        log.string("user_id", "usr_12345")
           .string("email", "john@example.com")
           .string("ip_address", "192.168.1.100")
           .bool("is_admin", false)
           .number("session_duration", 3600);
    });
    
    logger.error_with("Database connection failed", |log| {
        log.string("host", "localhost")
           .number("port", 5432)
           .string("database", "myapp_prod")
           .string("error_code", "CONN_TIMEOUT");
    });
    
    println!("\n=== Child Logger ===\n");
    
    // Child logger example
    let auth_logger = logger.child("auth");
    let db_logger = logger.child("database");
    
    auth_logger.info("Authentication module initialized");
    db_logger.warn("Connection pool running low");
    
    println!("\n=== No Colors Version ===\n");
    
    // Without colors
    let plain_logger = Logger::new("plain-app")
        .with_formatter(Box::new(PrettyFormatter::new().with_no_colors()));
    
    plain_logger.info("This message has no colors");
    plain_logger.error("Error without colors");
}