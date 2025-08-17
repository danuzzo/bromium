use log::{Level, LevelFilter, Metadata, Record};
use pyo3::prelude::*;
use std::sync::Mutex;

static LOGGER: BromiumLogger = BromiumLogger;
static LOG_LEVEL: Mutex<LevelFilter> = Mutex::new(LevelFilter::Debug);

struct BromiumLogger;

impl log::Log for BromiumLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let level = LOG_LEVEL.lock().unwrap();
        metadata.level() <= *level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            println!("{}: - bromium - {} - {}", timestamp, record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

pub fn init_logger() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Trace))
            .expect("Failed to initialize logger");
    });
}

pub fn set_log_level_internal(level: LevelFilter) {
    let mut log_level = LOG_LEVEL.lock().unwrap();
    *log_level = level;
    log::set_max_level(level);
}

#[pyfunction]
pub fn set_log_level(level: LogLevel) -> PyResult<()> {
    set_log_level_internal(level.into());
    log::info!("Log level set to: {:?}", level);
    Ok(())
}

#[pyfunction]
pub fn get_log_level() -> PyResult<String> {
    let level = LOG_LEVEL.lock().unwrap();
    Ok(format!("{:?}", *level))
}