use crate::app_run;
use crate::file_monitor::wait_until_update;
use log::info;
use std::path::Path;
use tokio::task;
pub async fn thread_start(profile: String, sampling_rate: String) {
    let full_path = profile.clone();
    let directory_path = full_path
        .rsplit_once('/')
        .map(|(dir, _)| dir.to_string()) // 获取目录部分并转换为 String
        .unwrap_or(full_path); // 如果没有找到 '/'，返回原始路径
    info!("directory_path={}", directory_path);
    let file_monitor_handle =
        task::spawn_blocking(move || wait_until_update(Path::new(&directory_path)));

    let run_thread_handle = task::spawn_blocking(move || app_run(&sampling_rate));

    // 等待两个任务完成
    let _ = file_monitor_handle.await.unwrap();
    let _ = run_thread_handle.await.unwrap();
}
