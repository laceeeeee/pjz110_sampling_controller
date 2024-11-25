use crate::app_run;
use crate::file_monitor::wait_until_update;
use log::info;
use std::path::Path;
pub async fn thread_start(profile: String, sampling_rate: String) {
    let full_path = profile.clone();
    let directory_path = full_path
        .rsplit_once('/')
        .map(|(dir, _)| dir.to_string()) // 获取目录部分并转换为 String
        .unwrap_or(full_path); // 如果没有找到 '/'，返回原始路径
    info!("directory_path={}", directory_path);
    // 使用 tokio::spawn_blocking 来启动阻塞任务
    let file_monitor_handle = tokio::spawn(async move {
        tokio::task::spawn_blocking(move || wait_until_update(Path::new(&directory_path)))
            .await
            .expect("Failed to run file monitor")
    });

    let run_thread_handle = tokio::spawn(async move {
        tokio::task::spawn_blocking(move || app_run(&sampling_rate))
            .await
            .expect("Failed to run app run")
    });

    // 等待两个任务完成
    let _ = file_monitor_handle.await;
    let _ = run_thread_handle.await;
}
