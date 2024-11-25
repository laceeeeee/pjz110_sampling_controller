// use crate::GLOBAL_MATCHES;
use crate::read::read_profile;
use anyhow::Result;
use inotify::{Inotify, WatchMask};
use log::info;
pub fn wait_until_update(path: &std::path::Path) -> Result<()> {
    let mut inotify = Inotify::init()?;
    info!("Inotify instance initialized");

    // 添加监视器，监控文件的修改事件
    inotify
        .watches()
        .add(path, WatchMask::MODIFY | WatchMask::CLOSE_WRITE)?;
    info!("Watch added for {:?}", path);

    loop {
        let mut buffer = [0; 1024];
        let _ = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error while reading events");
        reload_file(path.display().to_string());
    }
}

fn reload_file(path: String) {
    let full_path = format!("{}/games.toml", path);
    let _ = read_profile(full_path);
}
