use crate::rose::Rose;

use crate::rose::{ColorFormat, Colors};

use super::ctl;

pub async fn theme(colors: &Rose) {
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

    let mut handles = vec![];
    for command in que.iter() {
        handles.push(ctl(command.to_vec()));
    }
    for handle in handles {
        handle.await;
    }
}
