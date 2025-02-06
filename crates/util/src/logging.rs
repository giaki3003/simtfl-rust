// util/src/logging.rs

use log::{debug, error, info, warn, LevelFilter};
use simple_logger::SimpleLogger;

/// Initializes the global logger.
pub fn init_logger() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info) // Set the log level to INFO
        .init()
        .unwrap();
}

/// Logs an informational message.
pub fn log_info(message: &str) {
    info!("{}", message);
}

/// Logs a warning message.
pub fn log_warn(message: &str) {
    warn!("{}", message);
}

/// Logs an error message.
pub fn log_error(message: &str) {
    error!("{}", message);
}

/// Logs a debug message.
pub fn log_debug(message: &str) {
    debug!("{}", message);
}
