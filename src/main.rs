// main.rs
#[allow(dead_code)] // TODO: remove when all functions are properly referenced
mod args;
mod functions;
mod types;
mod utils;

use args::Args;
use utils::dirs::tmp_editor_directory;
use utils::proc::*;

use clap::Parser;

fn main() {
    let args = Args::parse();
    if args.help.is_some() {
        println!("TODO");
        std::process::exit(0);
    }

    let action = args.action.as_ref().map(|s| s.as_str()).unwrap_or("usage");
    let subject = args
        .subject
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("screen");

    let what = format!("{} {}", action, subject);

    // specify path to screenshot file
    // TODO: maybe this should be timestamped?
    // TODO: maybe this should be configurable?
    let temp_file = tmp_editor_directory() + "/grimblast.png";

    let actions: Vec<_> = vec![
        // TODO: make sure those actions receive the args they need
        ("active", functions::active_action(&what, &temp_file)),
        ("copy", functions::copy_action(&what, &temp_file)),
    ];

    for (action, func) in actions {
        // TODO: this doesn't allow mix'n matching arguments
        // we want to be able to use copy active and active copy to reach the same result
        // maybe some kind of list for priority arguments that'll be executed first when passed?
        if action == args.action.as_ref().map(|s| s.as_str()).unwrap_or("usage") {
            func().expect("Failed to perform action");
            break;
        }
    }

    wait();
}
