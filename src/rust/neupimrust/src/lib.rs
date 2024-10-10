use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
pub mod allocator;
pub mod global_config;
pub mod global_counts;
pub mod instruction;
pub mod no_icnt;
pub mod settings;
pub mod tensor;
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
/// 初始化日志记录器
///
/// # 参数
///
/// * `level` - 日志级别
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
