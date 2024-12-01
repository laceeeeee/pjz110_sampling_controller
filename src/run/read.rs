use crate::GLOBAL_MATCHES;
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;

pub fn read_profile(file: &str) -> Result<()> {
    // let config_str = fs::read_to_string(file)?;
    // let re = Regex::new(r#""(.*?)""#)?;
    let config_str =
        fs::read_to_string(file).with_context(|| format!("Failed to read file: {}", file))?;
    let re = Regex::new(r#""(.*?)""#).with_context(|| "Failed to compile regex")?;

    // let re = Regex::new(r#""([^"]*)""#)?;
    // 找到所有匹配的内容
    let matches = re.find_iter(&config_str);
    let mut global_matches = GLOBAL_MATCHES.lock();
    global_matches.clear();
    for mat in matches {
        let trimmed_str = mat.as_str().trim_matches('"'); // 去除双引号
        global_matches.push(trimmed_str.to_string());
    }
    Ok(())
}
