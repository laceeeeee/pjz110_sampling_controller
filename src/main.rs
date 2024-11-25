mod read;
mod utils;
use std::time::Duration;
use std::{env, fs, process, thread};

use crate::read::read_profile;
use crate::utils::get_top_app::get_topapp_pid_and_name;
use crate::utils::run_cmd::set_sampling_rate;
use anyhow::Result;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
pub static GLOBAL_MATCHES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn main() -> Result<()> {
    // 从文件中读取TOML内容
    let self_pid = process::id();
    let _ = fs::write("/dev/cpuset/background/cgroup.procs", self_pid.to_string());
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("提供的参数数量小于2，请提供至少2个参数，分别为配置文件路径，采样率");
        return Ok(());
    }
    let rs = read_profile(args[1].clone());
    if rs.is_err() {
        println!("出错啦读取文件");
        return Ok(());
    }
    print_app_list();
    let _ = run(&args[2]);
    Ok(())
}
fn print_app_list() {
    let global_matches = GLOBAL_MATCHES.lock();
    println!("以下是你在fas-rs的games.toml配置的包名:");
    for match_str in global_matches.iter() {
        println!("{}", match_str);
    }
}

fn judge_list_app(name: String, rate: &str) -> bool {
    let global_matches = GLOBAL_MATCHES.lock();
    for match_str in global_matches.iter() {
        if name == *match_str {
            println!(
                "检测到需要改变触控采样率的app: {}\n 触控采样率:{}",
                name, rate
            );
            set_sampling_rate(rate);
            return true;
        }
    }
    false
}

fn run(rate: &str) -> Result<()> {
    let mut global_package = String::new();
    loop {
        let (_, name) = get_topapp_pid_and_name()?;

        if global_package == name {
            thread::sleep(Duration::from_millis(1000));
            continue;
        }
        global_package = name.clone();
        let rs = judge_list_app(name.clone(), rate);
        if rs {
            continue;
        }
        println!("检测到日常app: {}", name);
        set_sampling_rate("120");
        thread::sleep(Duration::from_millis(1000));
    }
}
