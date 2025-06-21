pub mod logger;
pub mod level;
pub mod formatter;
pub mod output;

pub use logger::Logger;
pub use level::Level;
pub use formatter::{Formatter, PrettyFormatter, JsonFormatter};
pub use output::Output;

pub fn create_logger(name: &str) -> Logger {
    Logger::new(name)
}