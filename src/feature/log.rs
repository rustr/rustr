use log::{LogRecord, LogLevel, LogMetadata, Log};
use log::LogLevel::*;
use dll::message::*;

pub struct RLogger;

impl Log for RLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Warn | Error => {
                    r_warn(&format!("{} - {} in {:?}",
                                    record.level(),
                                    record.args(),
                                    record.location()))
                }
                _ => {
                    r_message(&format!("{} - {} in {:?}",
                                       record.level(),
                                       record.args(),
                                       record.location()))
                }
            }
        }
    }
}
