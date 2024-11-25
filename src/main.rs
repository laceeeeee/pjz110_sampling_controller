mod utils;

use regex::Regex;
use std::time::Duration;
use std::{env, fs, process, thread};

use crate::utils::get_top_app::get_topapp_pid_and_name;
use anyhow::Result;

use once_cell::sync::Lazy;
use parking_lot::Mutex;
static GLOBAL_MATCHES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn main() -> Result<()> {
    // 从文件中读取TOML内容
    let self_pid = process::id();
    let _ = fs::write("/dev/cpuset/background/cgroup.procs", self_pid.to_string());
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("参数数量小于2，请提供至少一个参数。");
        return Ok(());
    }
    let rs = read_file(args[1].clone());
    if rs.is_err() {
        println!("出错啦读取文件");
        return Ok(());
    }
    pt_app();
    let _ = run();
    Ok(())
}
fn pt_app() {
    let global_matches = GLOBAL_MATCHES.lock();
    for match_str in global_matches.iter() {
        println!("{}", match_str);
    }
}
fn read_file(file: String) -> Result<()> {
    let config_str = fs::read_to_string(file)?;
    let re = Regex::new(r#""(.*?)""#)?;
    // let re = Regex::new(r#""([^"]*)""#)?;
    // 找到所有匹配的内容
    let matches = re.find_iter(&config_str);
    let mut global_matches = GLOBAL_MATCHES.lock();
    for mat in matches {
        let trimmed_str = mat.as_str().trim_matches('"'); // 去除双引号
        global_matches.push(trimmed_str.to_string());
    }
    Ok(())
}
fn run() -> Result<()> {
    let mut global_package = String::new();
    loop {
        let (_, name) = get_topapp_pid_and_name()?;

        if global_package == name {
            println!("相同包名，返回");
            thread::sleep(Duration::from_millis(1000));
            continue;
        }
        global_package = name.clone();
        println!("上次报名:{}\n当前报名:{}", global_package, name);

        let global_matches = GLOBAL_MATCHES.lock();
        for match_str in global_matches.iter() {
            if name == *match_str {
                println!("此处改变触控采样率");
                continue;
            }
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
