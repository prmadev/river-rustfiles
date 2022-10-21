use crate::rose::Rose;

use crate::rose::{ColorFormat, Colors};

use super::ctl;

pub fn theme(colors: &Rose) {
    let colors = colors.to_owned();
    let base = &colors.get(Colors::Base, ColorFormat::UnixHex);
    let gold = &colors.get(Colors::Gold, ColorFormat::UnixHex);
    let love = &colors.get(Colors::Love, ColorFormat::UnixHex);

    let que: Vec<Vec<&str>> = vec![
        vec!["background-color", base],
        vec!["border-color-focused", gold],
        vec!["border-color-unfocused", base],
        vec!["border-color-urgent", love],
        vec!["border-width", "5"],
    ];

    que.iter().for_each(|x| ctl(x));
}
