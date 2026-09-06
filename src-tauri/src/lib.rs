mod db;

use db::{Grant, NewGrant, Manuscript};

/**
 * Grant-related Tauri commands
 */
#[tauri::command]
fn add_grant(grant: NewGrant) -> Result<Grant, String> {
        println!("TAURI: add_grant called with grant: {:?}", grant);
    db::add_grant_to_database(grant)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_grants() -> Result<Vec<Grant>, String> {
    db::get_grants_from_database()
        .map_err(|e| e.to_string())
}

/**
 * Manuscript-related Tauri commands
 */
#[tauri::command]
fn get_manuscripts() -> Result<Vec<Manuscript>, String> {
    db::get_manuscripts_from_database()
        .map_err(|e| e.to_string())
}

/**
 * Runs the Tauri application.
 */
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_grant, get_grants, get_manuscripts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}