mod inputs;
mod keybinds;
mod options;
mod tags;
mod theme;

use crate::config::Config;
use crate::rose::Rose;
use std::process::Command;

use self::{
    inputs::inputs,
    keybinds::{keyboard, mouse},
    options::options,
    tags::tags,
    theme::theme,
};

pub fn run(conf: Config, colors: &Rose) {
    mouse();
    keyboard(&conf);
    inputs(&conf);
    options();
    tags();
    theme(colors);
}

fn ctl(args: &Vec<&str>) {
    Command::new("riverctl")
        .args(args)
        .spawn()
        .expect(&format!("could not run riverctl {:#?}", args));
    // println!("\n:: |> \t{:?}\n", &args);
}
