use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::Local;
use log::Record;
use env_logger::fmt::Formatter;
use lazy_static::lazy_static;

// Global logger instance
lazy_static! {
    static ref LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
}

/// Initialize the logging system with file output
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Create logs directory if it doesn't exist
    let log_dir = get_log_directory()?;
    std::fs::create_dir_all(&log_dir)?;
    
    // Generate log file name with timestamp
    let log_file_path = log_dir.join(format!("bromium_{}.log", 
        chrono::Local::now().format("%Y%m%d_%H%M%S")));
    
    // Open log file
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&log_file_path)?;
    
    // Store file handle in global static
    *LOG_FILE.lock().unwrap() = Some(file);
    
    // Configure env_logger with custom format
    env_logger::Builder::from_default_env()
        .format(move |buf: &mut Formatter, record: &Record| {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            
            // Format: [timestamp] [LEVEL] [module::function:line] message
            writeln!(
                buf,
                "[{}] [{}] [{}:{}] {}",
                timestamp,
                record.level(),  // Use level directly instead of styling
                record.module_path().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )?;
            
            // Also write to file
            write_to_file(record)?;
            
            Ok(())
        })
        .filter_level(log::LevelFilter::Debug)
        .init();
    
    log::info!("Bromium logging initialized. Log file: {:?}", log_file_path);
    Ok(())
}

/// Write log entry to file
fn write_to_file(record: &Record) -> Result<(), std::io::Error> {
    if let Ok(mut log_file_guard) = LOG_FILE.lock() {
        if let Some(ref mut file) = *log_file_guard {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            writeln!(
                file,
                "[{}] [{}] [{}:{}] {}",
                timestamp,
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )?;
            file.flush()?;
        }
    }
    Ok(())
}

/// Get the appropriate log directory based on the platform
fn get_log_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        // On Windows, use %APPDATA%\Bromium\logs
        if let Ok(appdata) = std::env::var("APPDATA") {
            Ok(PathBuf::from(appdata).join("Bromium").join("logs"))
        } else {
            // Fallback to current directory
            Ok(PathBuf::from("./logs"))
        }
    }
    
    #[cfg(not(windows))]
    {
        // On other platforms, use ~/.local/share/bromium/logs
        if let Some(home) = dirs::home_dir() {
            Ok(home.join(".local").join("share").join("bromium").join("logs"))
        } else {
            // Fallback to current directory
            Ok(PathBuf::from("./logs"))
        }
    }
}

/// Macro for logging XPath operations with context
#[macro_export]
macro_rules! log_xpath_operation {
    ($level:expr, $operation:expr, $context:expr, $($arg:tt)*) => {
        log::log!($level, "[XPATH_{}] {} - {}", $operation, $context, format!($($arg)*));
    };
}

/// Macro for logging UI automation operations with context
#[macro_export]
macro_rules! log_uiauto_operation {
    ($level:expr, $operation:expr, $element_info:expr, $($arg:tt)*) => {
        log::log!($level, "[UIAUTO_{}] {} - {}", $operation, $element_info, format!($($arg)*));
    };
}

/// Performance timer for logging operation durations
pub struct PerformanceTimer {
    start_time: std::time::Instant,
    operation_name: String,
}

impl PerformanceTimer {
    pub fn new(operation_name: &str) -> Self {
        log::debug!("[PERF] Starting operation: {}", operation_name);
        Self {
            start_time: std::time::Instant::now(),
            operation_name: operation_name.to_string(),
        }
    }
}

impl Drop for PerformanceTimer {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        log::debug!("[PERF] Completed operation '{}' in {:.3}ms", 
                   self.operation_name, duration.as_secs_f64() * 1000.0);
    }
}

/// Clean up old log files (keep only the last N files)
pub fn cleanup_old_logs(keep_count: usize) -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = get_log_directory()?;
    if !log_dir.exists() {
        return Ok(());
    }
    
    let mut log_files: Vec<_> = std::fs::read_dir(&log_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "log")
                .unwrap_or(false)
        })
        .collect();
    
    // Sort by modification time (newest first)
    log_files.sort_by_key(|entry| {
        entry.metadata()
            .and_then(|meta| meta.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    log_files.reverse();
    
    // Remove old files
    for old_file in log_files.iter().skip(keep_count) {
        if let Err(e) = std::fs::remove_file(old_file.path()) {
            log::warn!("Failed to remove old log file {:?}: {}", old_file.path(), e);
        } else {
            log::info!("Removed old log file: {:?}", old_file.path());
        }
    }
    
    Ok(())
}