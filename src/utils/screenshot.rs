use super::proc::die;
use std::process::Command;

// TODO: use libwayland and hyprland-rs for taking screenshots
// hyprland-rs can get the current output or active window geometry
// which we can pass to libwayland's screenshot function
pub fn take_screenshot(file: &str, geom: Option<&str>, output: Option<&str>) {
    let mut cmd = Command::new("grim");

    if let Some(geom) = geom {
        cmd.arg("-g").arg(geom);
    }

    if let Some(output) = output {
        cmd.arg("-o").arg(output);
    }

    cmd.arg(file);

    let output = cmd.output().expect("Failed to execute command");

    if !output.status.success() {
        die(Some("Unable to invoke grim"));
    }
}
