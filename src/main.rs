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

// use mimalloc::MiMalloc;
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    init_misc();
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        info!("提供的参数数量小于3，请提供至少3个参数，分别为配置文件路径，游戏APP采样率，日常全局采样率");
        return Ok(());
    }
    let profile = args[1].clone();
    let games_sampling_rate = args[2].clone();
    let default_sampling_rate = args[3].clone();
    let rs = read_profile(profile.clone());
    if rs.is_err() {
        info!("出错啦读取文件");
        return Ok(());
    }
    print_app_list(&games_sampling_rate, &default_sampling_rate);
    let _ = thread_start(profile, games_sampling_rate, default_sampling_rate).await;

    Ok(())
}

fn print_app_list(games_sampling_rate: &str, default_sampling_rate: &str) {
    info!(
        "进入游戏的采样率:{} 全局默认采样率:{}",
        games_sampling_rate, default_sampling_rate
    );
    let global_matches = GLOBAL_MATCHES.lock();
    info!("以下是你在fas-rs的games.toml配置的包名:");
    for match_str in global_matches.iter() {
        info!("{}", match_str);
    }
}

fn init_misc() {
    let _ = init_log();
    let self_pid = process::id();
    let _ = fs::write("/dev/cpuset/background/cgroup.procs", self_pid.to_string());
}
