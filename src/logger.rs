use anyhow::Result;
use log::info;
use flexi_logger::{DeferredNow, LogSpecification, Logger, Record};
use std::io::{self, prelude::*};
pub fn init_log() -> Result<()> {
    // env_logger::init();
    std::env::set_var("RUST_LOG", "info");
    let logger_spec = LogSpecification::info();
    Logger::with(logger_spec)
        .log_to_stdout()
        .format(log_format)
        .start()?;
    Ok(())
}

fn log_format(
    write: &mut dyn Write,
    now: &mut DeferredNow,
    record: &Record<'_>,
) -> Result<(), io::Error> {
    let time = now.format("%Y-%m-%d %H:%M:%S");
    write!(write, "[{time}] {}: {}", record.level(), record.args())
}
