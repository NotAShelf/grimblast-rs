use notify_rust::Notification;

pub fn notify(message: &str) {
    Notification::new()
        .summary("Grimblast")
        .body(message)
        .timeout(notify_rust::Timeout::Milliseconds(3000))
        .show()
        .unwrap();
}

pub fn notify_ok(message: &str) {
    match std::env::var("NOTIFY") {
        Ok(val) if val == "no" => (),
        _ => notify(message),
    }
}

pub fn notify_error(message: &str) {
    match std::env::var("NOTIFY") {
        Ok(val) if val == "yes" => {
            let title = std::env::args()
                .nth(1)
                .unwrap_or_else(|| "Screenshot".into());
            let msg = std::env::args()
                .nth(2)
                .unwrap_or_else(|| "Error taking screenshot with grim".into());
            notify(&format!("{} {}", title, msg));
        }
        _ => println!("{}", message),
    }
}
