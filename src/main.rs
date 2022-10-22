//! RiverWM Dotfiles in Rust
//!
//! River window manager requires an executible to be configured
//! so I ended up writing it rust. previously I made a Go version.
//!
//! The old version (written in Go) available at [River-Dotfiles](https://github.com/amirography/River-Dotfiles)

mod config;
mod river;
mod rose;
mod startup;
mod wall;

use std::sync::Arc;

use config::Config;
use rose::Rose;
use tokio;

#[tokio::main]
async fn main() {
    // setting the configurations
    let conf = Arc::new(Config::new());

    let colors: Arc<Rose> = Arc::new(Rose::build(rose::Palletes::Default));

    // setting the wallpaper
    wall::set(&conf.wallpaper);

    let color1 = Arc::clone(&colors);
    let color2 = Arc::clone(&colors);

    let mut handles = vec![];
    handles.push(tokio::task::spawn(async move {
        river::run(Arc::clone(&conf), color1).await;
    }));

    // river::run(conf, colors);
    handles.push(tokio::task::spawn(async move {
        startup::run(color2).await;
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}
