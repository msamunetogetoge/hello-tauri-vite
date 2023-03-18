// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command() -> String {
    "I was invoked from JS!".to_string()
}

#[derive(Debug, serde::Serialize)]
struct User {
    name: String,
    age: u8,
}

#[derive(Debug, serde::Serialize)]
struct UserDescription {
    name: String,
    age: u8,
    class: String,
}

fn create_users() -> Vec<User> {
    let user1: User = User {
        name: "user1".into(),
        age: 12,
    };
    let user2: User = User {
        name: "user2".into(),
        age: 21,
    };

    let user3: User = User {
        name: "user3".into(),
        age: 41,
    };
    vec![user1, user2, user3]
}
#[tauri::command]
fn fetch_user() -> Vec<User> {
    create_users()
}

/*
エラー出るけど多分簡単
 */
#[tauri::command]
fn get_user_description(name: String) -> UserDescription {
    let users = create_users();
    let user = users.iter().find(|&x| x.name == name).unwrap();
    UserDescription {
        name: user.name.clone(),
        age: user.age,
        class: "hoge".to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            my_custom_command,
            fetch_user,
            get_user_description,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
