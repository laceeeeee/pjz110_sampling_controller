use crate::set_sampling_rate;
use crate::shared::file_monitor::wait_until_update;
use crate::shared::get_top_app::get_topapp_pid_and_name;
use crate::GLOBAL_MATCHES;
use anyhow::Result;
use log::info;
use std::path::Path;
use std::thread;
use tokio::time::Duration;
pub async fn thread_start(
    profile: String,
    games_sampling_rate: String,
    default_sampling_rate: String,
) -> Result<()> {
    // 启动文件监控任务
    let file_monitor = tokio::spawn(async move { wait_until_update(Path::new(&profile)) });

    // 启动运行线程任务
    let run_thread =
        tokio::spawn(async move { app_run(&games_sampling_rate, &default_sampling_rate) });

    // 等待两个任务完成
    let _ = file_monitor.await?;
    let _ = run_thread.await?;

    Ok(())
}

fn judge_list_app(name: &str, rate: &str) -> bool {
    let global_matches = GLOBAL_MATCHES.lock();
    for match_str in global_matches.iter() {
        if name == *match_str {
            info!("目标app: {} 触控采样率:{}", name, rate);
            set_sampling_rate(rate);
            return true;
        }
    }
    false
}

fn app_run(games_sampling_rate: &str, default_sampling_rate: &str) -> Result<()> {
    let mut global_package = String::new();
    loop {
        let (_, name) = get_topapp_pid_and_name()?;

        if global_package == name {
            thread::sleep(Duration::from_millis(1000));
            continue;
        }
        global_package = name.clone();
        let rs = judge_list_app(&global_package, games_sampling_rate);
        if rs {
            continue;
        }
        info!("日常app: {}", name);
        set_sampling_rate(default_sampling_rate);        
    }
}
