
use std::collections::HashMap;

use strum::{EnumIter, IntoEnumIterator};

use crate::utils::now;

pub enum TomatoStatus {
    Work,
    Reset,
    Block,
    Wait,
}

#[derive(Debug, EnumIter, PartialEq, Hash, Eq)]
pub enum TomatoHook {
    Start,
}

type Plugin =Box<dyn Fn(& Tomato, TomatoHook)>;

pub struct Tomato {
    pub status: TomatoStatus,
    pub loop_times: u16,
    pub start_at: Option<String>,
    pub start_at_current: Option<String>,
    plugins: HashMap<TomatoHook, Vec<Plugin>>
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
            start_at: None,
            start_at_current: None,
            plugins
        }
    }

    pub fn add_plugin(&mut self, plugin_tuple: (TomatoHook, Vec<Plugin>)) -> usize {
        let (hook, plugins) = plugin_tuple;
        if plugins.len() > 0 {
            let mut new_plugins: Vec<Plugin> = vec![];
            let mut old_plugins = self.plugins.get(&hook);
            if let Some(old)  = old_plugins  {
            
            };
        };
        0
    }

    fn start(&mut self) {
        self.status = TomatoStatus::Work;
    }
}

pub trait TomatoPlayer {
    async fn run(&mut self);
}

impl TomatoPlayer for Tomato {
    async fn run(&mut self) {
        match self.status {
            TomatoStatus::Wait => {
                self.start()
            },
            TomatoStatus::Work => todo!(),
            TomatoStatus::Reset => todo!(),
            TomatoStatus::Block => todo!(),
        }
    }
}