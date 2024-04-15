use chrono::{DateTime, Local, ParseError, Utc};

use crate::tomato::{Tomato, TomatoHook, TomatoStatus};

/// 以`YYYY-MM-DD HH:mm:ss`格式打印当前时间
pub fn now() -> String {
    Local::now().format("%F %T").to_string()
}

/// 批量执行当前`hook`的插件列表
pub fn exec_plugins(tomato: &Tomato, hook: TomatoHook) {
    if let Some(before_start_fns) =  tomato.plugins.get(&hook){
        for handler in before_start_fns {
            handler(tomato, TomatoHook::BeforeSetup);
        }
    }
}

/// 番茄钟启动
pub fn setup(tomato: &mut Tomato) {
    exec_plugins(tomato, TomatoHook::BeforeSetup);
    tomato.status = TomatoStatus::Work;
    tomato.setup_at = now();
    exec_plugins(tomato, TomatoHook::Setup);

    exec_plugins(tomato, TomatoHook::BeforeWork);
    tomato.work_at = now();
    exec_plugins(tomato, TomatoHook::Work);

}

fn time_has_expired(start: &str, during: u64) -> Result<bool, ParseError> {
    let start_at = start.parse::<DateTime<Utc>>()?;
    let during_sec = (during * 60) as i64;
    let has_expired = (Utc::now() - start_at).num_seconds() >= during_sec;
    Ok(has_expired)
}

pub fn run_with_work(tomato: &mut Tomato) -> Result<(), ParseError>{
    let work_has_expired = time_has_expired(&tomato.work_at, tomato.config.work_time_min)?;
    if work_has_expired {
        exec_plugins(tomato, TomatoHook::BeforeReset);
        tomato.status = TomatoStatus::Reset;
        tomato.reset_at = now();
        exec_plugins(tomato, TomatoHook::Reset);
    } else {
        exec_plugins(tomato, TomatoHook::Work);
    };
    Ok(())
}

pub fn run_with_reset(tomato: &mut Tomato) -> Result<(), ParseError> {
    let reset_has_expired = time_has_expired(&tomato.reset_at, tomato.config.reset_time_min)?;
    if reset_has_expired {
        exec_plugins(tomato, TomatoHook::BeforeWork);
        tomato.status = TomatoStatus::Work;
        tomato.reset_at = now();
        exec_plugins(tomato, TomatoHook::Work);
    } else {
        exec_plugins(tomato, TomatoHook::Reset);
    };
    Ok(())
}