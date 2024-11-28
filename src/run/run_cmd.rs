//From ab/SummerSK/puff
use std::process::Command;

pub fn set_sampling_rate(sampling_rate: &str) {
    let _ = Command::new("nohup")
        .args([
            "touchHidlTest",
            "-c",
            "wo",
            "0",
            "182",
            sampling_rate,
            ">/dev/null",
            "2>&1",
            "&",
        ])
        .output();
}
