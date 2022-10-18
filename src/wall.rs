use std::process::Command;

pub fn set(path: &str) {
    Command::new("swaybg")
        .args(["-i", path])
        .spawn()
        .expect("failed to run swaybg");
}
