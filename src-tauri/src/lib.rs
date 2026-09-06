mod db;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
//fn db(name: &str) -> String { format!("You have written {}!",)}

#[tauri::command]
fn db_call(name: &str) -> String {
    db::create_database(name);
    format!("You have written {}!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![db_call])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
