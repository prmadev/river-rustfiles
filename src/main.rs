mod config;
mod river;
mod rose;
mod startup;
mod wall;

use config::Config;
use rose::Rose;

fn main() {
    // setting the configurations
    let conf = Config::new();

    let colors = Rose::build(rose::Palletes::Default);

    // setting the wallpaper
    wall::set(&conf.wallpaper);

    river::run(conf, &colors);
    startup::run(&colors);
}
