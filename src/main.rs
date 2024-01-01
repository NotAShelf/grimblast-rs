mod args;
#[allow(dead_code)] // TODO: remove when all functions are properly referenced
mod utils;

use utils::dirs::*;
use utils::notify::*;
use utils::proc::*;
use utils::screenshot::*;

use args::Args;
use clap::Parser;

use std::process::Command;

fn main() {
    let args = Args::parse();
    if args.help {
        println!("TODO");
        std::process::exit(0);
    }

    match args.action.as_ref().map(|s| s.as_str()) {
        Some("check") => {
            println!("Checking if required tools are installed. If something is missing, install it to your system and make it available in PATH...");
            check("grim");
            check("slurp");
            check("hyprctl");
            check("hyprpicker");
            check("wl-copy");
            std::process::exit(0);
        }

        Some("active") => {
            // run hyprctl to get the focused window
            let output = Command::new("hyprctl")
                .arg("activewindow")
                .arg("-j")
                .output()
                .expect("Failed to execute command");

            // parse the output
            let focused: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            let geom: String = format!(
                "{},{} {}x{}",
                focused["at"][0], focused["at"][1], focused["size"][0], focused["size"][1]
            );
            let app_id: String = focused["class"].as_str().unwrap().to_string();
            let what = format!("{} window", app_id);

            // Take the screenshot
            take_screenshot(&what, Some(&geom), None);
        }

        Some("screen") => {
            // TODO
        }

        Some("output") => {
            // TODO
        }
        Some("area") => {
            // TODO
        }
        Some("window") => {
            die(Some("Subject 'window' is now included in 'area'"));
        }
        Some(unknown) => {
            die(Some(&format!(
                "Unknown subject to take a screen shot from: {}",
                unknown
            )));
        }
        None => {
            // TODO
        }
    }

    wait();

    match args.action.as_ref().map(|s| s.as_str()) {
        Some("copy") => {
            // TODO
        }

        Some("save") => {
            // TODO
        }

        Some("edit") => {
            // TODO
        }

        Some("copysave") => {
            // TODO
        }
        _ => {
            die(Some("Error taking screenshot with grim"));
        }
    }
}
