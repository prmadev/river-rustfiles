use dirs;

pub struct Config {
    pub wallpaper: String,
    pub terminal: String,
    pub launcher: String,
    pub password_manager: String,
    pub clipboard_manager: String,
    pub editor: String,
}
pub struct Places {}
impl Places {
    pub fn config() -> String {
        String::from(
            dirs::config_dir()
                .expect("couldn't find the config folder!")
                .to_str()
                .expect(""),
        )
    }

    // pub fn home() -> String {
    //     String::from(
    //         dirs::home_dir()
    //             .expect("couldn't find the home folder!")
    //             .to_str()
    //             .expect(""),
    // )
    // }
}

impl Config {
    pub fn new() -> Config {
        Config {
            wallpaper: format!("{}", Places::config() + "/wallpaper"),
            terminal: String::from("kitty"),
            launcher: String::from("rofi -show drun"),
            editor: String::from("zoxofi"),
            clipboard_manager: String::from("clipman pick -t rofi"),
            password_manager: String::from("rofi-rbw"),
        }
    }
}
