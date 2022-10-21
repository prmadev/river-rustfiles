use crate::config::{Config, Places};

use super::ctl;

pub fn mouse() {
    let mut que: Vec<Vec<&str>> = vec![];

    que.push(vec![
        "map-pointer",
        "normal",
        "Super",
        "BTN_LEFT",
        "move-view",
    ]);

    que.push(vec![
        "map-pointer",
        "normal",
        "Super",
        "BTN_RIGHT",
        "resize-view",
    ]);

    que.iter().for_each(|x| ctl(x));
}

pub fn keyboard(conf: &Config) {
    static MAP: &str = "map";
    static SPAWN: &str = "spawn";
    static NORMAL: &str = "normal";
    static FOCUS_VIEW: &str = "focus-view";
    static FOCUS_OUTPUT: &str = "focus-output";
    static SEND_TO_OUTPUT: &str = "send-to-output";
    static SEND_LAYOUT_CMD: &str = "send-layout-cmd";
    static RIVERTILE: &str = "rivertile";
    static NEXT: &str = "next";
    static PREV: &str = "previous";

    let file: String = format!("{}/river/init", Places::config());

    let que: Vec<Vec<&str>> = vec![
        vec![MAP, NORMAL, "Super", "R", SPAWN, &file],
        vec![MAP, NORMAL, "Super", "Return", SPAWN, &conf.terminal],
        vec![MAP, NORMAL, "Super", "D", SPAWN, &conf.launcher],
        vec![MAP, NORMAL, "Super", "P", SPAWN, &conf.password_manager],
        vec![MAP, NORMAL, "Super", "V", SPAWN, &conf.clipboard_manager],
        vec![MAP, NORMAL, "Super+Shift", "Return", SPAWN, &conf.editor],
        vec![MAP, NORMAL, "Super", "N", SPAWN, &conf.editor],
        vec![MAP, NORMAL, "Super", "J", FOCUS_VIEW, NEXT],
        vec![MAP, NORMAL, "Super", "K", FOCUS_VIEW, PREV],
        vec![MAP, NORMAL, "Super", "space", "zoom"],
        vec![MAP, NORMAL, "Super", "Q", "close"],
        vec![MAP, NORMAL, "Super", "Period", FOCUS_OUTPUT, "next"],
        vec![MAP, NORMAL, "Super", "Comma", FOCUS_OUTPUT, "previous"],
        vec![MAP, NORMAL, "Super+Shift", "Period", SEND_TO_OUTPUT, NEXT],
        vec![MAP, NORMAL, "Super+Shift", "Comma", SEND_TO_OUTPUT, PREV],
        vec![
            MAP,
            NORMAL,
            "Super",
            "H",
            SEND_LAYOUT_CMD,
            RIVERTILE,
            "main-ratio -0.05",
        ],
        vec![
            MAP,
            NORMAL,
            "Super",
            "L",
            SEND_LAYOUT_CMD,
            RIVERTILE,
            "main-ratio +0.05",
        ],
        vec![
            MAP,
            NORMAL,
            "Super+Alt+Shift",
            "H",
            "resize",
            "horizontal -100",
        ],
        vec![
            MAP,
            NORMAL,
            "Super+Alt+Shift",
            "J",
            "resize",
            "vertical 100",
        ],
        vec![
            MAP,
            NORMAL,
            "Super+Alt+Shift",
            "K",
            "resize",
            "vertical -100",
        ],
        vec![
            MAP,
            NORMAL,
            "Super+Alt+Shift",
            "L",
            "resize",
            "horizontal 100",
        ],
        vec![MAP, NORMAL, "Super+Shift", "F", "toggle-float"],
        vec![MAP, NORMAL, "Super", "F", "toggle-fullscreen"],
        vec![
            MAP,
            NORMAL,
            "Super",
            "Up",
            "send-layout-cmd",
            "rivertile",
            "main-location top",
        ],
        vec![
            MAP,
            NORMAL,
            "Super",
            "Right",
            "send-layout-cmd",
            "rivertile",
            "main-location right",
        ],
        vec![
            MAP,
            NORMAL,
            "Super",
            "Down",
            "send-layout-cmd",
            "rivertile",
            "main-location bottom",
        ],
        vec![
            MAP,
            NORMAL,
            "Super",
            "Left",
            "send-layout-cmd",
            "rivertile",
            "main-location left",
        ],
        vec![
            MAP,
            NORMAL,
            "None",
            "XF86AudioMedia",
            SPAWN,
            "playerctl play-pause",
        ],
        vec![
            MAP,
            NORMAL,
            "None",
            "XF86AudioPlay",
            SPAWN,
            "playerctl play-pause",
        ],
        vec![
            MAP,
            NORMAL,
            "None",
            "XF86AudioPrev",
            SPAWN,
            "playerctl previous",
        ],
        vec![
            MAP,
            NORMAL,
            "None",
            "XF86AudioNext",
            SPAWN,
            "playerctl next",
        ],
        vec![
            MAP,
            NORMAL,
            "None",
            "XF86AudioRaiseVolume",
            SPAWN,
            "pactl set-sink-volume @DEFAULT_SINK@ +5%",
        ],
        vec![
            MAP,
            NORMAL,
            "None",
            "XF86AudioLowerVolume",
            SPAWN,
            "pactl set-sink-volume @DEFAULT_SINK@ -5%",
        ],
        vec![
            MAP,
            NORMAL,
            "None",
            "XF86AudioMute",
            SPAWN,
            "pactl set-sink-mute @DEFAULT_SINK@ toggle",
        ],
        vec![
            MAP,
            NORMAL,
            "None",
            "XF86MonBrightnessUp",
            SPAWN,
            "brightnessctl s 5+",
        ],
        vec![
            MAP,
            NORMAL,
            "None",
            "XF86MonBrightnessDown",
            SPAWN,
            "brightnessctl s 5-",
        ],
    ];

    que.iter().for_each(|x| ctl(x));
}
