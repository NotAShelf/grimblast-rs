use std::process::Command;

// TODO: use libwayland and hyprland-rs for taking screenshots
// hyprland-rs can get the current output or active window geometry
// which we can pass to libwayland's screenshot function
pub fn take_screenshot(
    file: &str,
    geom: Option<&str>,
    output: Option<&str>,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("grim");

    if let Some(geom) = geom {
        if !geom.is_empty() {
            cmd.arg("-g").arg(geom);
        }
    }

    if let Some(output) = output {
        if !output.is_empty() {
            cmd.arg("-o").arg(output);
        }
    }

    cmd.arg(file);

    let output = cmd.output()?;
    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.contains("transparency") {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unable to invoke grim: {}", stderr),
            )));
        }
    }

    // if file is "-", store the screenshot in a file
    if file == "-" {
        std::fs::write(path, output.stdout)?;
        println!("Screenshot saved to: {}", path);
    }

    Ok(())
}
