// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::vec;
use serde::Deserialize;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

//elements structure
#[derive(Deserialize, Debug)]
struct Element {
    component: String,
    component_lineno: u32,
    content: String,
    id: String,
    component_no: u32
}

#[tauri::command]
fn auto_save(elements: Vec<Vec<Element>>) {
    //try and save this to a file
    for element in elements {
        println!("{:?}", element)
    }
    println!("");
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let save = CustomMenuItem::new("save".to_string(), "Save");
    let darkmode = CustomMenuItem::new("darkmode".to_string(), "Dark Mode");
    let filemenu = Submenu::new("File", Menu::new().add_item(quit).add_item(save));
    let viewmenu = Submenu::new("View", Menu::new().add_item(darkmode));

    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(filemenu)
        .add_submenu(viewmenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => {
                    std::process::exit(0);
                }
                "darkmode" => {
                    println!("triggered")
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![auto_save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}