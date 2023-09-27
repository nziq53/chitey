use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process;

pub fn save_pid() {
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push("chitey_api_server_pid.txt");
    let mut file = File::create(tmp_dir.clone()).expect("create failed");
    let pid = process::id().to_string();
    file.write_all(pid.as_bytes()).expect("write failed");
}

pub fn load_pid() -> u32 {
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push("chitey_api_server_pid.txt");
    let mut file = File::open(tmp_dir).expect("open failed");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("load failed");
    let pid: u32 = contents.parse().unwrap();
    println!("{}", pid);
    pid
}
