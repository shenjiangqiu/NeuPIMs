use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
pub mod settings;
pub mod no_icnt;
#[no_mangle]
pub extern "C" fn test_rust(a: i32, b: i32) -> i32 {
    let c = a + b;
    info!("{} + {} = {}", a, b, c);
    c
}

#[repr(C)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}
#[no_mangle]
pub extern "C" fn init_logger(level: LogLevel) {
    let level = LevelFilter::from(level);

    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .from_env_lossy(),
        )
        .try_init()
        .unwrap_or_else(|err| {
            eprintln!("Failed to init logger: {}", err);
        });
    info!("Logger initialized");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn main() {
        println!("Hello, world!");
    }
}
