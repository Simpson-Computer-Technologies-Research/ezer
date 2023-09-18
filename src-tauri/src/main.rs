// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, process::Command};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![install])
        .invoke_handler(tauri::generate_handler![write_to_json])
        .invoke_handler(tauri::generate_handler![read_from_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn install(os: String, cmds: Vec<String>) -> bool {
    let mut message: bool = false;

    cmds.iter().for_each(|cmd: &String| match os.as_str() {
        "windows" => {
            let args = ["/C", cmd];

            message = Command::new("cmd").args(args).status().unwrap().success();
        }
        "mac" => {
            let args: Vec<&str> = cmd.split(" ").collect::<Vec<&str>>();

            message = Command::new("sudo").args(args).status().unwrap().success();
        }
        _ => {
            println!("Unsupported OS");
        }
    });

    return message;
}

#[tauri::command]
fn write_to_json(file: String, data: String) -> bool {
    let path = env::current_dir().unwrap();
    let path = path.join(file);

    std::fs::write(path, data).is_ok()
}

#[tauri::command]
fn read_from_json(file: String) -> String {
    let path = env::current_dir().unwrap();
    let path = path.join(file);

    std::fs::read_to_string(path).unwrap()
}
