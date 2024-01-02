use std::io::Write;
use std::process::{Command, Stdio};

use libwayshot::{CaptureRegion, WayshotConnection};
use std::path::Path;

pub fn take_screenshot(
    region: CaptureRegion,
    file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = WayshotConnection::new()?;
    let image = conn.screenshot(region, true)?;

    image.save(Path::new(file))?;

    Ok(())
}
