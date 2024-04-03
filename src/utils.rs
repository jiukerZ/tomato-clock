use chrono::Local;

/// Print the current time in the format `YYYY-MM-DD HH:mm:ss`
pub fn now() -> String {
    Local::now().format("%F %T").to_string()
}