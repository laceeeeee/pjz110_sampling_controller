use log::info;
use flexi_logger::{DeferredNow, LogSpecification, Logger, Record};
use anyhow::Result;
use std::{
    env, fs,
    io::{self, prelude::*},
    process,
};
pub fn init_log()-> Result<()> {
    env_logger::init();
    let logger_spec = LogSpecification::info();
    Logger::with(logger_spec)
        .log_to_stdout()
        .format(log_format)
        .start()?;

    // log_metainfo();
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

// fn log_metainfo() {
    // info!(
        // "fas-rs v{} {}, llvm-{}, rustc-{}, build by {} at {} on {},{},{}",
        // env!("CARGO_PKG_VERSION"),
        // build_type(),
        // env!("VERGEN_RUSTC_LLVM_VERSION"),
        // env!("VERGEN_RUSTC_SEMVER"),
        // env!("VERGEN_SYSINFO_USER"),
        // env!("VERGEN_BUILD_TIMESTAMP"),
        // env!("VERGEN_SYSINFO_NAME"),
        // env!("VERGEN_SYSINFO_OS_VERSION"),
        // env!("VERGEN_RUSTC_HOST_TRIPLE")
    // );
// }
