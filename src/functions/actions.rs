use crate::types::ActionFunc;
use crate::utils::screenshot::take_screenshot;

use serde_json::Value;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn active_action<'a>(_what: &'a str, screenshot_path: &'a str) -> ActionFunc<'a> {
    Box::new(move || {
        let output = Command::new("hyprctl")
            .arg("activewindow")
            .arg("-j")
            .output()
            .expect("Failed to execute command");

        let focused: Value = serde_json::from_slice(&output.stdout).unwrap();
        let geom: String = format!(
            "{},{} {}x{}",
            focused["at"][0], focused["at"][1], focused["size"][0], focused["size"][1]
        );
        let app_id: String = focused["class"].as_str().unwrap().to_string();
        let _what = format!("{} window", app_id);

        take_screenshot("-", Some(&geom), None, screenshot_path)?;

        Ok(())
    })
}

pub fn copy_action<'a>(what: &'a str, screenshot_path: &'a str) -> ActionFunc<'a> {
    Box::new(move || {
        let screenshot = fs::read(screenshot_path)?;
        let copy_result = Command::new("wl-copy")
            .arg("--type")
            .arg("image/png")
            .stdin(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                child.stdin.as_mut().unwrap().write_all(&screenshot)?;
                Ok(child.wait()?)
            });
        match copy_result {
            Ok(_) => println!("{} copied to buffer", what),
            Err(_) => eprintln!("Clipboard error"),
        }

        Ok(())
    })
}
