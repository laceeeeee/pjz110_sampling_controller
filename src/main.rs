mod run;
mod shared;
use crate::run::read::read_profile;
use crate::run::run_cmd::set_sampling_rate;
use crate::run::start::thread_start;
use crate::shared::logger::init_log;
use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::{env, fs, process};
pub static GLOBAL_MATCHES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn init_misc() {
    let _ = init_log();
    let self_pid = process::id();
    let _ = fs::write("/dev/cpuset/background/cgroup.procs", self_pid.to_string());
}

#[tokio::main]
async fn main() -> Result<()> {
    init_misc();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        info!("提供的参数数量小于2，请提供至少2个参数，分别为配置文件路径，采样率");
        return Ok(());
    }
    let profile = args[1].clone();
    let sampling_rate = args[2].clone();
    let rs = read_profile(profile.clone());
    if rs.is_err() {
        info!("出错啦读取文件");
        return Ok(());
    }
    print_app_list();
    let _ = thread_start(profile, sampling_rate).await;

    Ok(())
}

fn print_app_list() {
    let global_matches = GLOBAL_MATCHES.lock();
    info!("以下是你在fas-rs的games.toml配置的包名:");
    for match_str in global_matches.iter() {
        info!("{}", match_str);
    }
}
