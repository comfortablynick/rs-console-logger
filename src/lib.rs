//! This module defines a super simple logger that works with the `log` crate.
//! We don't need anything fancy; just basic log levels and the ability to
//! print to stderr. We therefore avoid bringing in extra dependencies just
//! for this functionality.
//! From: ripgrep
//!
//! Added by github.com/comfortablynick
//! - Init log with verbosity level

use log::{self, Log};

/// The simplest possible logger that logs to stderr.
///
/// This logger does no filtering. Instead, it relies on the `log` crate's
/// filtering via its global max_level setting.
#[derive(Debug)]
pub struct Logger(());

const LOGGER: &Logger = &Logger(());

impl Logger {
    /// Create a new logger that logs to stderr and initialize it as the
    /// global logger. If there was a problem setting the logger, then an
    /// error is returned.
    pub fn init(verbose: u8) -> Result<(), log::SetLoggerError> {
        let level = match verbose {
            0 => log::Level::Warn,
            1 => log::Level::Info,
            2 => log::Level::Debug,
            _ => log::Level::Trace,
        };
        log::set_max_level(level.to_level_filter());
        log::set_logger(LOGGER)
    }
}

impl Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        // We set the log level via log::set_max_level, so we don't need to
        // implement filtering here.
        true
    }

    fn log(&self, record: &log::Record) {
        match (record.file(), record.line()) {
            (Some(file), Some(line)) => {
                eprintln!(
                    "{}|{}|{}:{}: {}",
                    record.level(),
                    record.target(),
                    file,
                    line,
                    record.args()
                );
            }
            (Some(file), None) => {
                eprintln!(
                    "{}|{}|{}: {}",
                    record.level(),
                    record.target(),
                    file,
                    record.args()
                );
            }
            _ => {
                eprintln!("{}|{}: {}", record.level(), record.target(), record.args());
            }
        }
    }

    fn flush(&self) {
        // We use eprintln! which is flushed on every call.
    }
}
