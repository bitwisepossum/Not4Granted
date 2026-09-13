mod db;
mod models;
mod diesel_db;
mod schema;
mod settings;

use models::{Grant, NewGrant, Manuscript, NewManuscript, GrantQuery};

use crate::{diesel_db::GrantRow, schema::grant};

const DATABASE_URL: &str = "db-diesel.sqlite3";

/**
 * Grant-related Tauri commands
 */
/*
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
} */

// Diesel database functions

#[tauri::command]
fn get_filtered_grants(query: GrantQuery) -> Result<Vec<Grant>, String> {
    diesel_db::get_filtered_grants(
        DATABASE_URL,
        &query,
    )
}

#[tauri::command]
fn get_grant_by_id(grant_id: i64) -> Result<Grant, String> {
    diesel_db::get_grant_by_id(DATABASE_URL, grant_id)
        .and_then(|grant| {
            grant.ok_or_else(|| format!("Grant with id {} not found", grant_id))
        })
}

#[tauri::command]
fn get_all_grants() -> Result<Vec<Grant>, String> {
    diesel_db::get_all_grants(DATABASE_URL)
}

#[tauri::command]
fn update_grant(grant: Grant) -> Result<(), String> {
    diesel_db::update_grant(DATABASE_URL, &grant)
}

#[tauri::command]
fn delete_grant(grant_id: i64) -> Result<(), String> {
    diesel_db::delete_grant(DATABASE_URL, grant_id)
}

#[tauri::command]
fn add_grant(new_grant: NewGrant) -> Result<Grant, String> {
    diesel_db::add_grant(DATABASE_URL, &new_grant)
}

#[tauri::command]
fn get_grants() -> Result<Vec<Grant>, String> {
    diesel_db::get_all_grants(DATABASE_URL)
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
 * Settings-related Tauri commands
 */
#[tauri::command]
fn get_settings(
    app: tauri::AppHandle,
) -> Result<settings::Settings, String> {
    settings::load_settings(&app)
}

#[tauri::command]
fn save_settings(
    app: tauri::AppHandle,
    settings: settings::Settings,
) -> Result<settings::Settings, String> {
    settings::save_settings(&app, settings)
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
            update_grant,
            delete_grant,
            get_filtered_grants,
            get_all_grants,
            get_grant_by_id,
            add_manuscript, 
            get_manuscripts,
            get_manuscript_by_id,
            get_filtered_grants,
            delete_manuscript,
            update_manuscript,
            get_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}