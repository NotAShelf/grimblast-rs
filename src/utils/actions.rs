use hyprland::data::Client;
use hyprland::shared::HyprDataActiveOptional;
use libwayshot::{CaptureRegion, WayshotConnection};

pub fn get_active() -> Result<CaptureRegion, Box<dyn std::error::Error>> {
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
