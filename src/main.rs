mod tomato;
mod utils;

use clap::Parser;
use tokio::{self, runtime::Runtime, time::{self, Duration, Instant}};

use tomato::{Tomato, TomatoHook, TomatoPlayer};
use utils::now;


#[derive(Parser)]
#[command(name="tomato",version, author, about, long_about=None)]
struct Cli;


#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let mut tomato = Tomato::new();
    let p = |t: &Tomato, h: TomatoHook| {
        println!("Tomato setup!")
    };
    let len = tomato.add_plugin((TomatoHook::Setup, vec![Box::new(p)]));
    let mut interval = time::interval(time::Duration::from_secs(2));
    {
        loop {
            interval.tick().await;
            tomato.run().await;
        }
    }
}
