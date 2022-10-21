use super::ctl;

pub fn options() {
    const FLOAT_ADD: &str = "float-filter-add";
    const ID: &str = "app-id";
    const TITLE: &str = "title";
    const CSD_ADD: &str = "csd-filter-add";

    let que: Vec<Vec<&str>> = vec![
        // keyboard repeating rate
        vec!["set-repeat", "50", "300"],
        // Make certain views start floating
        vec![FLOAT_ADD, ID, "Rofi"],
        vec![FLOAT_ADD, ID, "float"],
        vec![FLOAT_ADD, ID, "popup"],
        vec![FLOAT_ADD, ID, "pinentry-qt"],
        vec![FLOAT_ADD, ID, "pinentry-gtk"],
        vec![FLOAT_ADD, TITLE, "Picture-in-Picture"],
        vec![FLOAT_ADD, ID, "launcher"],
        vec![CSD_ADD, ID, "Rofi"],
        vec![CSD_ADD, ID, "launcher"],
        // mouse stuff
        vec!["focus-follows-cursor", "normal"],
        vec!["set-cursor-warp", "on-output-change"],
        // layout related
        vec!["attach-mode", "bottom"], // new window's open at the end of stack instead of on top
        vec!["default-layout", "rivertile"], // default layouting engine
    ];

    que.iter().for_each(|x| ctl(x));
}
