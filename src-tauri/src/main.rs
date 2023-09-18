// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, process::Command};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![install,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn install(os: String, cmds: Vec<String>, password: String) -> bool {
    let mut message: bool = false;

    cmds.iter().for_each(|cmd: &String| match os.as_str() {
        "windows" => {
            let args = ["/C", cmd];

            message = Command::new("cmd").args(args).status().unwrap().success();
        }
        "mac" => {
            // Command to run the command in sudo
            let sudo_cmd = format!("echo {} | sudo -S {}", &password, cmd);

            let args: [&str; 2] = ["-c", &sudo_cmd];

            message = Command::new("sh").args(args).status().unwrap().success();
        }
        _ => {
            println!("Unsupported OS");
        }
    });

    return message;
}
