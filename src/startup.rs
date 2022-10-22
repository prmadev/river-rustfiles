use super::Rose;
use dirs::config_dir;
use rustix::process;
use std::{
    ffi::OsString,
    fs::{self, read_dir},
};

use crate::rose::{ColorFormat, Colors};
use std::sync::Arc;
use tokio::{process::Command, task};
pub async fn run(colors: Arc<Rose>) {
    let wall_path = format!("{}/{}", config_dir().unwrap().display(), "wallpaper");

    let handles = vec![
        task::spawn(async move { restart_proc("waybar", vec![]).await }),
        // Setting wallpaper (use 'ln -P' to an image here.)
        task::spawn(
            async move { restart_proc("swaybg", vec!["-m", "fill", "-i", &wall_path]).await },
        ),
        // something I saw others did. I don't know why.
        task::spawn(async move {
            restart_proc(
                "mako",
                vec![
                    "--default-timeout",
                    "5000",
                    "--background-color",
                    &colors.get(Colors::Surface, ColorFormat::RGBHex),
                    "--border-color",
                    &colors.get(Colors::Surface, ColorFormat::RGBHex),
                    "--border-size",
                    "0",
                    "--font",
                    "monospace",
                    "--padding",
                    "10",
                    "--width",
                    "350",
                ],
            )
            .await
        }),
        task::spawn(async move {
            restart_proc(
                "rivertile",
                vec!["-view-padding", "05", "-outer-padding", "05"],
            )
            .await
        }),
        task::spawn(async move {
            restart_proc(
                "wl-paste",
                vec!["-t", "text", "--watch", "clipman", "store"],
            )
            .await
        }),
        task::spawn(async move {
            start_proc(
                "dbus-update-activation-environment",
                vec![
                    "SEATD_SOCK",
                    "DISPLAY",
                    "WAYLAND_DISPLAY",
                    "XDG_SESSION_TYPE",
                    "XDG_CURRENT_DESKTOP",
                ],
            )
            .await
        }),
    ];

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn start_proc(process_name: &str, args: Vec<&str>) {
    Command::new(process_name).args(&args).spawn().expect("");
}

async fn restart_proc(process_name: &str, args: Vec<&str>) {
    kill_procs(process_name);
    Command::new(process_name)
        .args(&args)
        .spawn()
        .expect(&format!(
            "could not run process: {} {:#?}",
            process_name, args
        ));
}

fn kill_procs(process_name: &str) {
    let proc_reader = match read_dir("/proc") {
        Ok(proc) => proc,
        Err(e) => {
            println!("could not read /proc: {}", e);
            return;
        }
    };

    proc_reader
        .filter(|ent| match ent {
            Ok(file) => match file.file_type() {
                Ok(ty) if is_proc_dir(&file.file_name()) => ty.is_dir(),
                _ => false,
            },
            Err(_) => false,
        })
        .map(|ent| ent.expect("this should not be happening!"))
        .filter(|x| {
            match fs::read_to_string(format!(
                "{:}/cmdline",
                format!("{:?}", x.path()).trim_matches('"')
            )) {
                Ok(ent) => ent.contains(process_name),
                Err(_) => false,
            }
        })
        .for_each(|x| match x.file_name().to_str() {
            Some(name) => kill(name.parse::<u32>().expect("this should not be happening!")),
            None => (),
        });
}

fn kill(pid: u32) {
    unsafe {
        _ = process::kill_process(process::Pid::from_raw(pid).unwrap(), process::Signal::Kill);
    }
}
fn is_proc_dir(file_name: &OsString) -> bool {
    match file_name.to_owned().into_string() {
        Ok(name) => name
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect::<Vec<char>>()
            .is_empty(),
        _ => false,
    }
}
