use std::process::Command;

pub fn set_sampling_rate(rate: &str) {
    let _ = Command::new("touchHidlTest")
        .args(["-c", "wo", "0", "182", rate])
        .output();
}
