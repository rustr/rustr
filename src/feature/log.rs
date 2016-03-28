use log::{LogRecord, LogLevel, LogMetadata,Log};
use log::LogLevel::*;
use dll::message::*;

pub struct RLogger;

impl Log for RLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
        	match record.level(){
        		Warn | Error=> r_warn(&format!("{} - {}", record.level(), record.args())),
        		_ => r_message(&format!("{} - {}", record.level(), record.args()))
        	}
        }
    }
}
