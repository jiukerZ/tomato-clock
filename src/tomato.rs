
use std::{collections::HashMap, future::Future};

use strum::{EnumIter, IntoEnumIterator};

use crate::utils::{now, exec_plugins};

pub enum TomatoStatus {
    Work,
    Reset,
    Block,
    Wait,
}

#[derive(Debug, EnumIter, PartialEq, Hash, Eq)]
pub enum TomatoHook {
    BeforeSetup,
    Setup,
}

type Plugin =Box<dyn Fn(& Tomato, TomatoHook)>;

pub struct Tomato {
    pub status: TomatoStatus,

    /// 循环了多少次完整的时钟周期（不包括本轮）。
    pub loop_times: u16,

    /// 启动时间
    pub setup_at:String,

    pub work_at: String,

    pub reset_at: String,

    pub block_at: String,

    pub resume_at: String,

    pub plugins: HashMap<TomatoHook, Vec<Plugin>>
}

impl Tomato {
    pub fn new() -> Self {
        let mut plugins = HashMap::new();
        for hook in TomatoHook::iter()  {
            plugins.insert(hook, vec![]);
        }
        Tomato {
            status: TomatoStatus::Wait,
            loop_times: 0,
            setup_at: "".to_string(),
            plugins,
            work_at: "".to_string(),
            reset_at: "".to_string(),
            block_at: "".to_string(),
            resume_at: "".to_string(),
        }
    }
}

pub trait TomatoPlayer {
    /// 定时任务中，指定间隔时间内（2s）会触发一次，可以
    /// 可以通过`run`调整番茄钟的状态
    async fn run(&mut self);

    fn add_plugin(&mut self, plugin_tuple: (TomatoHook, Vec<Plugin>)) -> usize;

    fn setup(&mut self);
}

impl TomatoPlayer for Tomato {
    async fn run(&mut self) {
        match self.status {
            TomatoStatus::Wait => {
                self.setup();
            },
            TomatoStatus::Work => todo!(),
            TomatoStatus::Reset => todo!(),
            TomatoStatus::Block => todo!(),
        }
    }

     fn add_plugin(&mut self, plugin_tuple: (TomatoHook, Vec<Plugin>)) -> usize {
        let (hook, plugins) = plugin_tuple;
        if plugins.len() > 0 {
            let  old_plugins = self.plugins.get_mut(&hook);
            if let Some(old)  = old_plugins  {
                old.extend(plugins);
            };
        };
        self.plugins.len()
    }

    fn setup(&mut self) {
        exec_plugins(self, TomatoHook::BeforeSetup);
        self.status = TomatoStatus::Work;
        self.setup_at = now();
        exec_plugins(self, TomatoHook::Setup);
    }
}