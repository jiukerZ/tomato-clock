use chrono::Local;

use crate::tomato::{Tomato, TomatoHook};

/// Print the current time in the format `YYYY-MM-DD HH:mm:ss`.
pub fn now() -> String {
    Local::now().format("%F %T").to_string()
}

/// Batch executes the specified hook function.
pub fn exec_plugins(tomato: &Tomato, hook: TomatoHook) {
    if let Some(before_start_fns) =  tomato.plugins.get(&hook){
        for handler in before_start_fns {
            handler(tomato, TomatoHook::BeforeSetup);
        }
    }
}