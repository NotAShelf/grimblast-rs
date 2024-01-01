use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "NotAShelf",
    version = "v0.1.0",
    about,
    long_about = None,
    disable_help_flag = true
)]

pub struct Args {
    #[clap(short, long)]
    notify: bool,

    #[clap(short, long)]
    cursor: bool,

    #[clap(short, long)]
    freeze: bool,

    #[clap(short, long, default_value = "no")]
    wait: String,

    #[clap(short, long)]
    scale: Option<String>,

    #[clap(short, long, default_value = "-1")]
    hyprpicker_pid: i32,

    pub action: Option<String>,

    subject: Option<String>,

    file: Option<String>,

    file_editor: Option<String>,

    #[clap(long, action = clap::ArgAction::HelpLong)]
    pub help: bool,
}
