// 定义一个函数，执行命令并解析输出
use anyhow::Result;
use dumpsys_rs::Dumpsys;
pub fn get_topapp_pid_and_name() -> Result<(String, String)> {
    // let output = Command::new("dumpsys")
    // .args(&["activity", "lru"])
    // .output()?;
    let result = Dumpsys::new("activity").unwrap().dump(&["lru"]);
    // println!("{:?}", result);
    // if !output.status.success() {
    // return Ok(("".to_string(), "".to_string()));
    // }

    // let result = String::from_utf8_lossy(&output.stdout);
    let output = result?;
    // println!("{}", output);
    let top_index = output.find(" TOP").unwrap_or(0);
    let end_index = output[top_index..].find('/').unwrap_or(output.len());

    if top_index + 13 <= output.len() && top_index + end_index <= output.len() {
        // let top_app = &output[top_index + 13..top_index + end_index];
        let top_app = output
            .get(top_index + 13..top_index + end_index)
            .unwrap_or("");
        let mut parts = top_app.split(':');
        let pid = parts.next().unwrap_or("").trim().to_string();
        let name = parts.next().unwrap_or("").trim().to_string();

        return Ok((pid, name));
    }
    // 可以选择返回一个错误、默认值或者处理这种情况的其他方式
    // 息屏时
    Ok(("".to_string(), "".to_string()))
}

/*师范:
fn main() -> io::Result<()> {
    // 调用函数并接收返回的两个值
    let (pid, name) = get_topapp_pid_and_name()?;

    // 打印这两个值
    println!("pid: -{}-", pid);
    println!("topappname: -{}-", name);

    Ok(())
}
*/
