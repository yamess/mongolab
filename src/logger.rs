use log::LevelFilter;
use log4rs::append::{
    console::{ConsoleAppender, Target},
    rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller,
    rolling_file::policy::compound::trigger::size::SizeTrigger,
    rolling_file::policy::compound::CompoundPolicy,
    rolling_file::RollingFileAppender,
};
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::{json::JsonEncoder, pattern::PatternEncoder};
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Config;

#[derive(Debug)]
pub struct FileAppender;

impl FileAppender {
    pub fn build(
        file_trigger_size: u64,
        log_file: &str,
        log_file_count: u32,
    ) -> RollingFileAppender {
        let archive_file_pattern = Self::get_archive_file_pattern(log_file);
        let trigger = SizeTrigger::new(file_trigger_size);
        let roller = FixedWindowRoller::builder()
            .base(0)
            .build(archive_file_pattern.as_str(), log_file_count)
            .unwrap();
        let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
        RollingFileAppender::builder()
            .encoder(Box::new(JsonEncoder::new()))
            .build(log_file, Box::new(policy))
            .unwrap()
    }

    fn get_archive_file_pattern(file_name: &str) -> String {
        let parts: Vec<&str> = file_name.split('/').collect();
        let file_name = parts.last().unwrap();
        let extension = file_name.split('.').last().unwrap();
        let name = file_name.replace(extension, "");
        let archive_file = format!("archives/{}{{}}.{}", name, extension);
        parts[..parts.len() - 1].join("/") + "/" + &archive_file
    }
}

#[derive(Debug)]
pub struct ConsoleLogAppender;

impl ConsoleLogAppender {
    pub fn build(target: Target) -> ConsoleAppender {
        let pattern = PatternEncoder::new(
            "[{X(request_id)(Internal):<16}] - [{d(%Y-%m-%d %H:%M:%S)(utc)} - \
        {h({l}):<5.5} - {T} - {M}:{L}]: {m}{n}",
        );
        ConsoleAppender::builder()
            .target(target)
            .encoder(Box::new(pattern))
            .build()
    }
}

fn get_logger_level(level: &str) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Off,
    }
}

pub fn init_logger(app_name: &str, console_log_level: &str, file_log_level: &str, file_path: &str) {
    let file_log_level = get_logger_level(file_log_level);
    let console_log_level = get_logger_level(console_log_level);

    let stdout = ConsoleLogAppender::build(Target::Stdout);
    let file = FileAppender::build(1024 * 1024 * 10, file_path, 10);

    let logger_config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(console_log_level)))
                .build("stdout", Box::new(stdout)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(file_log_level)))
                .build("file", Box::new(file)),
        )
        .logger(Logger::builder().build(app_name, LevelFilter::Trace))
        .logger(Logger::builder().build("griot", LevelFilter::Trace))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Off),
        )
        .unwrap();
    log4rs::init_config(logger_config).unwrap();
    log::info!("Logger initialized");
}
