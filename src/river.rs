mod keybinds;

use crate::config::Config;
use std::process::Command;

use self::keybinds::mouse;

#[allow(unused)]
pub fn run(conf: Config) {
    mouse();
}

fn ctl(args: Vec<&str>) {
    Command::new("riverctl")
        .args(&args)
        .spawn()
        .expect(&format!("could not run riverctl {:#?}", args));
}
