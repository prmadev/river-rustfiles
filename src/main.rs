mod config;
mod river;
mod wall;

use config::Config;

fn main() {
    // setting the configurations
    let conf = Config::new();

    // setting the wallpaper
    wall::set(&conf.wallpaper);

    river::run(conf)
}
