use clap::ValueEnum;

pub static mut LOGLEVEL: LogLevel = LogLevel::Info;

pub fn set_log_level(log_level: LogLevel) {
    unsafe {
        LOGLEVEL = log_level;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, ValueEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

pub fn enabled(level: LogLevel) -> bool {
    unsafe {
        level >= LOGLEVEL
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        if logger::enabled(LogLevel::Trace) {
            println!("[TRACE] {}", format!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if logger::enabled(LogLevel::Debug) {
            println!("[DEBUG] {}", format!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if logger::enabled(logger::LogLevel::Info) {
            println!("[INFO] {}", format!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if logger::enabled(LogLevel::Warn) {
            println!("[WARN] {}", format!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if logger::enabled(LogLevel::Error) {
            println!("[ERROR] {}", format!($($arg)*));
        }
    }
}