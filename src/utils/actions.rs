use hyprland::data::{Client, Monitor, Monitors};
use hyprland::shared::HyprDataActiveOptional;
use hyprland::shared::HyprData;
use libwayshot::{CaptureRegion, WayshotConnection};

pub fn get_active_region() -> Result<CaptureRegion, Box<dyn std::error::Error>> {
    let client_option = Client::get_active()?;
    if let Some(client) = client_option {
        let region = CaptureRegion {
            x_coordinate: client.at.0 as i32,
            y_coordinate: client.at.1 as i32,
            width: client.size.0 as i32,
            height: client.size.1 as i32,
        };
        Ok(region)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No active client found",
        )))
    }
}

// get the name of the focused focused name by intering the monitors struct
// until we find a monitor that has focused set to true
pub fn get_focused_monitor() -> Result<String, Box<dyn std::error::Error>> {
  let monitors = Monitors::get()?;
  let get_monitors = match monitors.iter().find(|m| m.focused == true) {
      Some(monitor) => Ok(monitor.name.clone()),
      None => Err("No focused monitor found".into())
  };

  get_monitors
}
