mod db;
mod models;

use models::{Grant, NewGrant, Manuscript, NewManuscript};

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

#[tauri::command]
fn get_grant_by_id(grant_id: i64) -> Result<Grant, String> {
    db::get_grant_by_id_from_database(grant_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_grant(grant: Grant) -> Result<(), String> {
    db::update_grant_in_database(grant)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_grant(grant_id: i64) -> Result<(), String> {
    db::delete_grant_from_database(grant_id)
        .map_err(|e| e.to_string())
}

/**
 * Manuscript-related Tauri commands
 */
#[tauri::command]
fn add_manuscript(manuscript: NewManuscript) -> Result<Manuscript, String> {
    db::add_manuscript_to_database(manuscript)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_manuscripts() -> Result<Vec<Manuscript>, String> {
    db::get_manuscripts_from_database()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_manuscript_by_id(manuscript_id: i64) -> Result<Manuscript, String> {
    db::get_manuscript_by_id_from_database(manuscript_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_manuscript(manuscript: Manuscript) -> Result<(), String> {
    db::update_manuscript_in_database(manuscript)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_manuscript(manuscript_id: i64) -> Result<(), String> {
    db::delete_manuscript_from_database(manuscript_id)
        .map_err(|e| e.to_string())
}

/**
 * Runs the Tauri application.
 */
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_grant, 
            get_grants, 
            get_grant_by_id, 
            update_grant,
            delete_grant,
            add_manuscript, 
            get_manuscripts,
            get_manuscript_by_id,
            delete_manuscript,
            update_manuscript
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}