#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::State;

struct Counter(Mutex<usize>);

use serde::{Deserialize, Serialize};
#[allow(unused)]
use tauri::RunEvent;
use tauri::{
    webview::{PageLoadEvent, WebviewWindowBuilder},
    App, Emitter, Listener, Runtime, WebviewUrl,
};
use tauri_plugin_fs::FsExt;

#[tauri::command]
fn increment(counter: State<'_, Counter>) -> usize {
    let mut c = counter.0.lock().unwrap();
    *c += 1;
    *c
}

#[tauri::command]
fn decrement(counter: State<'_, Counter>) -> usize {
    let mut c = counter.0.lock().unwrap();
    *c -= 1;
    *c
}

#[tauri::command]
fn reset(counter: State<'_, Counter>) -> usize {
    let mut c = counter.0.lock().unwrap();
    *c = 0;
    *c
}

#[tauri::command]
fn get(counter: State<'_, Counter>) -> usize {
    *counter.0.lock().unwrap()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Counter(Mutex::new(0)))
        .invoke_handler(tauri::generate_handler![increment, decrement, reset, get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
