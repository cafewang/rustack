use chrono::{Local};

pub fn lprintf(msg: &str, file: &str, line: u32) {
    let fmt = "%Y年%m月%d日 %H:%M:%S";

    let now = Local::now().format(fmt);

    println!("{}-{:?}-{:?} {msg}", now, file, line);
}

pub fn dump(data: &[u8], len: u16) {
    // todo 
}