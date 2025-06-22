use cappie::{Logger, Level};
use cappie::formatter::{FlexibleFormatter, ComponentType, ComponentPosition};

fn main() {
    println!("=== FLEXIBLE FORMATTER EXAMPLES ===\n");

    // Example 1: Default format
    println!("1. Default format:");
    let logger1 = Logger::new("app")
        .with_formatter(Box::new(FlexibleFormatter::new()));
    
    logger1.info("This is the default format");
    logger1.info_with("With additional fields", |log| {
        log.string("user", "alice")
           .number("count", 42);
    });
    println!();

    // Example 2: Custom order - Message first, then level and time
    println!("2. Message first format:");
    let logger2 = Logger::new("api")
        .with_formatter(Box::new(
            FlexibleFormatter::new()
                .clear_components()
                .add_message(ComponentPosition::Start, None, None, None)
                .add_level(ComponentPosition::AfterMessage, Some("\x1b[31m".to_string()), Some(" ".to_string()), None)
                .add_timestamp(ComponentPosition::AfterLevel, None, Some(" [".to_string()), Some("]".to_string()))
                .add_custom_text(": ", ComponentPosition::AfterTime, None)
                .add_fields(ComponentPosition::End, None, None, None)
        ));
    
    logger2.error("Database connection failed");
    logger2.warn_with("High memory usage detected", |log| {
        log.number("usage_mb", 1024)
           .string("process", "web-server");
    });
    println!();

    // Example 3: Minimal format - Just level and message
    println!("3. Minimal format:");
    let logger3 = Logger::new("minimal")
        .with_formatter(Box::new(
            FlexibleFormatter::new()
                .clear_components()
                .add_level(ComponentPosition::Start, Some("\x1b[33m".to_string()), Some("[".to_string()), Some("]".to_string()))
                .add_message(ComponentPosition::AfterLevel, None, Some(" ".to_string()), None)
        ));
    
    logger3.warn("This is a warning");
    logger3.info("This is info");
    println!();

    // Example 4: Time at the end format
    println!("4. Time at the end:");
    let logger4 = Logger::new("backend")
        .with_formatter(Box::new(
            FlexibleFormatter::new()
                .clear_components()
                .add_logger_name(ComponentPosition::Start, Some("\x1b[36m".to_string()), Some("[".to_string()), Some("]".to_string()))
                .add_level(ComponentPosition::AfterName, Some("\x1b[32m".to_string()), Some(" ".to_string()), None)
                .add_custom_text(": ", ComponentPosition::AfterLevel, None)
                .add_message(ComponentPosition::AfterLevel, None, None, None)
                .add_timestamp(ComponentPosition::End, Some("\x1b[90m".to_string()), Some(" (at ".to_string()), Some(")".to_string()))
                .add_fields(ComponentPosition::AfterMessage, None, Some(" | ".to_string()), None)
        ));
    
    logger4.info("Server started successfully");
    logger4.debug_with("Processing request", |log| {
        log.string("method", "GET")
           .string("path", "/api/users")
           .number("response_time", 45);
    });
    println!();

    // Example 5: Colorful custom format
    println!("5. Highly colorful format:");
    let logger5 = Logger::new("colorful")
        .with_formatter(Box::new(
            FlexibleFormatter::new()
                .clear_components()
                .add_custom_text("🚀 ", ComponentPosition::Start, Some("\x1b[95m".to_string())) // Bright magenta emoji
                .add_timestamp(ComponentPosition::Start, Some("\x1b[94m".to_string()), Some("[".to_string()), Some("]".to_string())) // Bright blue time
                .add_logger_name(ComponentPosition::AfterTime, Some("\x1b[96m".to_string()), Some(" {".to_string()), Some("}".to_string())) // Bright cyan name
                .add_level(ComponentPosition::AfterName, Some("\x1b[93m".to_string()), Some(" <".to_string()), Some(">".to_string())) // Bright yellow level
                .add_custom_text(" → ", ComponentPosition::AfterLevel, Some("\x1b[97m".to_string())) // Bright white arrow
                .add_message(ComponentPosition::AfterLevel, Some("\x1b[92m".to_string()), None, None) // Bright green message
                .add_fields(ComponentPosition::End, Some("\x1b[91m".to_string()), Some(" 📊 ".to_string()), None) // Bright red fields with emoji
        ));
    
    logger5.info("Application initialized");
    logger5.error_with("Connection timeout", |log| {
        log.string("host", "localhost")
           .number("port", 5432)
           .number("timeout_ms", 5000);
    });
    println!();

    // Example 6: No colors format
    println!("6. No colors format:");
    let logger6 = Logger::new("plain")
        .with_formatter(Box::new(
            FlexibleFormatter::new()
                .with_no_colors()
                .clear_components()
                .add_timestamp(ComponentPosition::Start, None, Some("TIME:".to_string()), None)
                .add_logger_name(ComponentPosition::AfterTime, None, Some(" APP:".to_string()), None)
                .add_level(ComponentPosition::AfterName, None, Some(" LEVEL:".to_string()), None)
                .add_message(ComponentPosition::AfterLevel, None, Some(" MSG:".to_string()), None)
                .add_fields(ComponentPosition::End, None, Some(" DATA:".to_string()), None)
        ));
    
    logger6.info_with("User authentication", |log| {
        log.string("username", "john_doe")
           .bool("success", true);
    });
    println!();

    // Example 7: JSON-like but readable format
    println!("7. JSON-like readable format:");
    let logger7 = Logger::new("json-like")
        .with_formatter(Box::new(
            FlexibleFormatter::new()
                .clear_components()
                .add_custom_text("{ ", ComponentPosition::Start, None)
                .add_custom_text("\"timestamp\": \"", ComponentPosition::Start, Some("\x1b[36m".to_string()))
                .add_timestamp(ComponentPosition::Start, None, None, None)
                .add_custom_text("\", \"level\": \"", ComponentPosition::AfterTime, Some("\x1b[36m".to_string()))
                .add_level(ComponentPosition::AfterTime, Some("\x1b[33m".to_string()), None, None)
                .add_custom_text("\", \"logger\": \"", ComponentPosition::AfterLevel, Some("\x1b[36m".to_string()))
                .add_logger_name(ComponentPosition::AfterLevel, Some("\x1b[32m".to_string()), None, None)
                .add_custom_text("\", \"message\": \"", ComponentPosition::AfterName, Some("\x1b[36m".to_string()))
                .add_message(ComponentPosition::AfterName, Some("\x1b[97m".to_string()), None, None)
                .add_custom_text("\"", ComponentPosition::AfterMessage, Some("\x1b[36m".to_string()))
                .add_fields(ComponentPosition::End, Some("\x1b[35m".to_string()), Some(", \"fields\": { ".to_string()), Some(" }".to_string()))
                .add_custom_text(" }", ComponentPosition::End, None)
        ));
    
    logger7.warn_with("Rate limit exceeded", |log| {
        log.string("client_ip", "192.168.1.100")
           .number("requests_per_min", 150);
    });

    println!("\n=== END OF EXAMPLES ===");
}