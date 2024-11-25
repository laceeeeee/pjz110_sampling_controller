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
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error while reading events");
        for event in events {
            info!("Event: {:?}", event);
        }
    }
}
