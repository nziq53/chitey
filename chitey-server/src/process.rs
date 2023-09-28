use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process;

use crate::web_server::ChiteyError;

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
    // println!("{}", pid);
    pid
}

pub fn kill_server() -> Result<(), ChiteyError>{
    let pid = load_pid();
    #[cfg(target_os = "windows")]
    {
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::processthreadsapi::TerminateProcess;
        use winapi::um::winnt::PROCESS_ALL_ACCESS;

        unsafe {
            let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
            let mut ret: Result<(), ChiteyError> = Ok(()); 
            if process_handle.is_null() {
                ret = Err(ChiteyError::ServerKillError("Process does not exist".to_string()
            ));
            }

            if TerminateProcess(process_handle, 1) == 0 {
                ret =  Err(ChiteyError::ServerKillError(format!(
                    "プロセスを終了できません。エラーコード: {}",
                    std::io::Error::last_os_error()
                )));
            } else {
                ret = Ok(());
            }

            // プロセスハンドルをクローズ
            if !process_handle.is_null() {
                CloseHandle(process_handle);
            }
            return ret;
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Err(err) = nix::sys::signal::kill(
            nix::unistd::Pid::from_raw(pid as i32),
            nix::sys::signal::SIGKILL,
        ) {
            return Err(ChiteyError::ServerKillError(format!("Failed to kill process: {}", err)));
        }
    }
    // println!("kill success!!");
    Ok(())
}
