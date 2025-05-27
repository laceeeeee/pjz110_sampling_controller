use anyhow::Result;
use dumpsys_rs::Dumpsys;

pub fn get_topapp_pid_and_name() -> Result<(String, String)> {
    let output = Dumpsys::new("activity").unwrap().dump(&["lru"]);
    if output.is_err() {
        return Ok(("".to_string(), "".to_string()));
    }
    let output = output?;

    let top_index = output.find(" TOP").unwrap_or(0);
    let end_index = output[top_index..].find('/').unwrap_or(output.len());

    let top_app = output
        .get(top_index + 13..top_index + end_index)
        .unwrap_or("");
    let mut parts = top_app.split(':');
    let pid = parts.next().unwrap_or("").trim().to_string();
    let name = parts.next().unwrap_or("").trim().to_string();

    Ok((pid, name))
}
