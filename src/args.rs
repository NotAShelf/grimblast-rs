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
    pub notify: bool,

    #[clap(short, long)]
    pub cursor: bool,

    #[clap(short, long)]
    pub freeze: bool,

    #[clap(short, long, default_value = "no")]
    pub wait: String,

    #[clap(short, long)]
    pub scale: Option<String>,

    #[clap(short, long, default_value = "-1")]
    pub hyprpicker_pid: i32,

    pub action: Option<String>,

    pub subject: Option<String>,

    pub file: Option<String>,

    pub file_editor: Option<String>,

    #[clap(long, action = clap::ArgAction::HelpLong)]
    pub help: Option<bool>,
}
