use crate::utils::notify::notify_error;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn kill_hyprpicker() {
    match std::env::var("HYPRPICKER_PID") {
        Ok(pid) if pid != "-1" => {
            Command::new("kill")
                .arg(&pid)
                .spawn()
                .expect("Failed to execute command");
        }
        _ => {}
    }
}

/// Terminates the program.
///
/// This function kills the Hyprpicker process, sends an error notification with a custom message,
/// and then exits the program with a status code of 2. If killing Hyprpicker is not necessary,
/// then panic!() should be used instead.
///
/// # Arguments
///
/// * `msg` - An optional string slice that contains the error message to be displayed. If `None`,
/// the default message "Bye!" is used.
///
/// # Panics
///
/// This function does not return. It terminates the program immediately after being called.
pub fn die(msg: Option<&str>) {
    kill_hyprpicker();
    let message = msg.unwrap_or("Bye!");
    notify_error(&format!("Error: {}", message));
    std::process::exit(2);
}

pub fn check(command: &str) {
    let result = Command::new(command).arg("--version").output().is_ok();
    let result_str = if result { "OK" } else { "NOT FOUND" };
    println!("  {}: {}", command, result_str);
}

pub fn wait() {
    match std::env::var("WAIT") {
        Ok(wait) if wait != "no" => {
            let dur = Duration::from_secs(wait.parse().unwrap());
            thread::sleep(dur);
        }
        _ => {}
    }
}
