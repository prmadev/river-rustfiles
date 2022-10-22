mod inputs;
mod keybinds;
mod options;
mod tags;
mod theme;

use crate::config::Config;
use crate::rose::Rose;
use std::sync::Arc;
use tokio::process::Command;
use tokio::task;

use self::{
    inputs::inputs,
    keybinds::{keyboard, mouse},
    options::options,
    tags::tags,
    theme::theme,
};

pub async fn run(conf: Arc<Config>, colors: Arc<Rose>) {
    let mut handles = vec![];

    handles.push(task::spawn(async move {
        mouse().await;
    }));
    handles.push(task::spawn(async move {
        keyboard().await;
    }));
    handles.push(task::spawn(async move {
        inputs(conf).await;
    }));
    handles.push(task::spawn(async move {
        options().await;
    }));
    handles.push(task::spawn(async move {
        tags().await;
    }));
    handles.push(task::spawn(async move {
        theme(&colors).await;
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn ctl(args: Vec<&str>) {
    Command::new("riverctl")
        .args(&args)
        .spawn()
        .expect(&format!("could not run riverctl {:#?}", &args));
    // println!("\n:: |> \t{:?}\n", &args);
}
