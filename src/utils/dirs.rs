use std::env;
use std::fs;
use std::path::Path;

pub fn get_target_directory() -> String {
    let config_home = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| String::from("~/.config"));
    let user_dirs_dirs_path = format!("{}/user-dirs.dirs", config_home);

    let mut screenshots_dir = None;
    let mut pictures_dir = None;
    let mut home_dir = None;

    if Path::new(&user_dirs_dirs_path).exists() {
        let contents = fs::read_to_string(&user_dirs_dirs_path).expect("Unable to read file");

        for line in contents.lines() {
            if line.starts_with("XDG_SCREENSHOTS_DIR") {
                screenshots_dir = Some(line.split('=').nth(1).unwrap_or("").trim().to_string());
            } else if line.starts_with("XDG_PICTURES_DIR") {
                pictures_dir = Some(line.split('=').nth(1).unwrap_or("").trim().to_string());
            } else if line.starts_with("HOME") {
                home_dir = Some(line.split('=').nth(1).unwrap_or("").trim().to_string());
            }
        }
    }

    screenshots_dir.unwrap_or_else(|| pictures_dir.unwrap_or(home_dir.unwrap()))
}

pub fn tmp_editor_directory() -> String {
    // if tmp directory doesn't exist, or is not writable
    // exit with an error
    // else return the /tmp directory
    if Path::new("/tmp").exists() {
        String::from("/tmp")
    } else {
        panic!("Unable to find /tmp directory");
    }
}

pub fn env_editor_confirm() {
    match std::env::var("GRIMBLAST_EDITOR") {
        Ok(_) => println!("GRIMBLAST_EDITOR is set. Continuing..."),
        Err(_) => {
            println!("GRIMBLAST_EDITOR is not set. Defaulting to gimp");
            std::env::set_var("GRIMBLAST_EDITOR", "gimp");
        }
    }
}
