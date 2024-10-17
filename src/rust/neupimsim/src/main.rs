use neupimrust::{init_logger, LogLevel};
fn main() {
    init_logger(LogLevel::Info);
    neupimrust::run();
}

#[cfg(test)]
mod tests {
    use tracing::debug;
    #[test]
    fn test_init_logger() {
        super::init_logger(super::LogLevel::Debug);
        debug!("This is a debug message");
    }
}
