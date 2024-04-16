use std::process;

use chrono::{DateTime, Local, ParseError, Utc};
use inquire::Select;

use crate::tomato::{Tomato, TomatoHook, TomatoStatus};

pub fn exit_process() {
    process::exit(exitcode::OK);
}

/// 以`YYYY-MM-DD HH:mm:ss`格式打印当前时间
pub fn now() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S%Z").to_string()
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

fn time_has_expired(start: &str, during_min: i64, block_sec: i64) -> bool {
    let start_at = start.parse::<DateTime<Utc>>().unwrap();
    let during_sec = during_min * 60;
    let has_expired = (Utc::now() - start_at).num_seconds() >= (during_sec - block_sec);
    has_expired
}

pub fn run_with_work(tomato: &mut Tomato, block_sec: i64){
    let work_has_expired = time_has_expired(&tomato.work_at, tomato.config.work_time_min as i64, block_sec);
    if work_has_expired {
        // 切换到下一阶段
        exec_plugins(tomato, TomatoHook::BeforeReset);
        tomato.status = TomatoStatus::Reset;
        tomato.reset_at = now();
        exec_plugins(tomato, TomatoHook::Reset);
    } else {
        exec_plugins(tomato, TomatoHook::Work);
    };
}

pub fn run_with_reset(tomato: &mut Tomato, block_sec: i64)  {
    let reset_has_expired = time_has_expired(&tomato.reset_at, tomato.config.reset_time_min as i64, block_sec);
    if reset_has_expired {
        // 切换到下一阶段
        exec_plugins(tomato, TomatoHook::BeforeWork);
        tomato.status = TomatoStatus::Work;
        tomato.reset_at = now();
        exec_plugins(tomato, TomatoHook::Work);
    } else {
        exec_plugins(tomato, TomatoHook::Reset);
    };
}

/// 处理番茄钟运行时
pub fn handle_run(tomato: &mut Tomato, from: TomatoStatus, to: TomatoStatus) {
    let block_time_sec = if from == TomatoStatus::Block {
        let block_at = tomato.block_at.parse::<DateTime<Utc>>().expect("Failed to resolve the pause time");
        let resume_at = tomato.resume_at.parse::<DateTime<Utc>>().expect("Failed to resolve the recovery time");
        let sec = (resume_at - block_at).num_seconds();
        sec
    } else {0};

    if to == TomatoStatus::Work {
        run_with_work(tomato, block_time_sec);
    } else {
        run_with_reset(tomato, block_time_sec)
    }
}

pub fn handle_block(tomato: &mut Tomato) {
    if tomato.status == TomatoStatus::Block {
        return;
    };
    tomato.block_from = Some(tomato.status);
    tomato.block_at = now();
    tomato.status = TomatoStatus::Block;
    exec_plugins(tomato, TomatoHook::Block);

    let resume = "从暂停中恢复";
    let exit = "退出程序";

    let options = vec![resume, exit];
    let ans = Select::new("番茄钟暂停", options).with_starting_cursor(1).prompt();

    match ans {
        Ok(choice) => {
            if choice == exit {
                exit_process();
            }
        },
        Err(_) => println!("There was an error, please try again"),
    }
}

pub fn handle_resume(tomato: &mut Tomato) {
    tomato.resume_at = now();
    tomato.status = tomato.block_from.expect("Recover from the unknown!");
    handle_run(tomato, TomatoStatus::Block, TomatoStatus::Work);
}