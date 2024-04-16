mod tomato;
mod utils;

use std::{process, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use clap::Parser;
use tokio::{self, runtime::Runtime, sync::Mutex, time::{self, Duration, Instant} };

use tomato::{Tomato, TomatoHook, TomatoPlayer, TomatoStatus};
use utils::{handle_block, exit_process};


#[derive(Parser)]
#[command(name="tomato",version, author, about, long_about=None)]
struct Cli;


#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let tomato = Arc::new(Mutex::new(Tomato::new()));
    let config = tomato.lock().await.config;
    let tomato_clone = Arc::clone(&tomato);

    tokio::spawn(async move  {
        tokio::signal::ctrl_c().await.unwrap();
        let mut lock = tomato_clone.lock().await;
        if lock.status == TomatoStatus::Block {
            exit_process();
        }
        handle_block(&mut lock);
    });

    let mut interval = time::interval(time::Duration::from_secs(config.run_interval_sec + 10));
    {
        loop {
            interval.tick().await;
            tomato.lock().await.run().await;
        }
    }
}
