// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod server;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn dumb(name: &str) -> String {
    format!("Hey!!")
} 

#[tauri::command]
fn start_server() -> String {
    // tokio::spawn
    "Server starting...".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![dumb, start_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
