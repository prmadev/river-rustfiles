use super::ctl;
use crate::config::Config;

pub fn inputs(conf: &Config) {
    const INPUT: &str = "input";
    const ENABLED: &str = "enabled";

    let que: Vec<Vec<&str>> = vec![
        vec![INPUT, &conf.touch, "drag", ENABLED],
        vec![INPUT, &conf.touch, "tap", ENABLED],
        vec![INPUT, &conf.touch, "events", ENABLED],
        vec![INPUT, &conf.touch, "natural-scroll", ENABLED],
        vec![INPUT, &conf.touch, "scroll-method", "two-finger"],
    ];
    que.iter().for_each(|x| ctl(x));
}
