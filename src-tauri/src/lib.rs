mod db;
mod models;
mod diesel_db;
mod schema;
mod settings;

use models::{
    Grant, 
    NewGrant, 
    GrantQuery, 
    Manuscript, 
    NewManuscript,
    ManuscriptQuery, 
};

use crate::{diesel_db::GrantRow, schema::grant};

const DATABASE_URL: &str = "db-diesel.sqlite3";

/**
 * Grant-related Tauri commands
 */

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
fn get_filtered_manuscripts(query: ManuscriptQuery) -> Result<Vec<Manuscript>, String> {
    diesel_db::get_filtered_manuscripts(
        DATABASE_URL,
        &query,
    )
}

#[tauri::command]
fn get_manuscript_by_id(manuscript_id: i64) -> Result<Manuscript, String> {
    diesel_db::get_manuscript_by_id(DATABASE_URL, manuscript_id)
        .and_then(|manuscript| {
            manuscript.ok_or_else(|| format!("Manuscript with id {} not found", manuscript_id))
        })
}

#[tauri::command]
fn get_all_manuscripts() -> Result<Vec<Manuscript>, String> {
    diesel_db::get_all_manuscripts(DATABASE_URL)
}

#[tauri::command]
fn update_manuscript(manuscript: Manuscript) -> Result<(), String> {
    diesel_db::update_manuscript(DATABASE_URL, &manuscript)
}

#[tauri::command]
fn delete_manuscript(manuscript_id: i64) -> Result<(), String> {
    diesel_db::delete_manuscript(DATABASE_URL, manuscript_id)
}

#[tauri::command]
fn add_manuscript(new_manuscript: NewManuscript) -> Result<Manuscript, String> {
    diesel_db::add_manuscript(DATABASE_URL, &new_manuscript)
}

#[tauri::command]
fn get_manuscripts() -> Result<Vec<Manuscript>, String> {
    diesel_db::get_all_manuscripts(DATABASE_URL)
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