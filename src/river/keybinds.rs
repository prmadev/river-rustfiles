use super::ctl;

pub fn mouse() {
    ctl(vec![
        "map-pointer",
        "normal",
        "Super",
        "BTN_LEFT",
        "move-view",
    ]);

    ctl(vec![
        "map-pointer",
        "normal",
        "Super",
        "BTN_RIGHT",
        "resize-view",
    ])
}

pub fn keyboard() {
    ctl(vec![])
}
