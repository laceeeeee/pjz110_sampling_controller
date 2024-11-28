use crate::run::read::read_profile;
use anyhow::Result;
use inotify::{Inotify, WatchMask};
use log::info;
pub fn wait_until_update(path: &std::path::Path) -> Result<()> {
    // let directory_path = profile
    // .rsplit_once('/')
    // .map(|(dir, _)| dir.to_string()) // 获取目录部分并转换为 String
    // .unwrap_or(profile); // 如果没有找到 '/'，返回原始路径
    let parent_path = path.parent().unwrap();

    let mut inotify = Inotify::init()?;
    info!("Inotify instance initialized");

    // 添加监视器，监控文件的修改事件
    inotify
        .watches()
        .add(parent_path, WatchMask::MODIFY | WatchMask::CLOSE_WRITE)?;
    info!("Watch added for {:?}", parent_path);
    // let path = path.display().to_string();

    let path_ref: &str = &path.display().to_string();
    loop {
        let mut buffer = [0; 1024];
        let _ = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error while reading events");
        reload_file(path_ref);
    }
}

fn reload_file(full_path: &str) {
    let _ = read_profile(full_path.to_string());
}
