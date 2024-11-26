use crate::app_run;
use crate::file_monitor::wait_until_update;
use anyhow::Result;
use std::path::Path;
pub async fn thread_start(profile: String, sampling_rate: String) -> Result<()> {
    // 使用 tokio::spawn_blocking 来启动阻塞任务
    let file_monitor_handle = tokio::spawn(async move {
        tokio::task::spawn_blocking(move || wait_until_update(Path::new(&profile))).await?
    });

    let run_thread_handle =
        tokio::spawn(
            async move { tokio::task::spawn_blocking(move || app_run(&sampling_rate)).await? },
        );

    // 等待两个任务完成
    let _ = file_monitor_handle.await;
    let _ = run_thread_handle.await;
    Ok(())
}
