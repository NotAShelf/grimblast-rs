use crate::utils::screenshot::take_screenshot;

use serde_json::Value;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio, Output};

fn process_output(output: &Output) -> String {
  // TODO: get focused window and its geometry from hyprland-rs
  let focused: Value = serde_json::from_slice(&output.stdout).unwrap();
  let geom: String = format!(
      "{},{} {}x{}",
      focused["at"][0], focused["at"][1], focused["size"][0], focused["size"][1]
  );

  geom
}

pub fn active_action(screenshot_path: &str) {
  let output = Command::new("hyprctl")
      .arg("activewindow")
      .arg("-j")
      .output()
      .expect("Failed to execute command");

  let geom = process_output(&output);

  take_screenshot("-", Some(&geom), None, screenshot_path).unwrap();
}
