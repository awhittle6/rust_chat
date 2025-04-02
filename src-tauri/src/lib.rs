// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod server;
mod client;
mod errors;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn dumb(name: &str) -> String {
    format!("Hey!!")
} 

#[tauri::command]
async fn start_server() -> Result<bool, errors::ApplicationError> {
    // tokio::spawn
    tokio::spawn(async move {
        if let Err(e) = server::start_server().await {
            println!("Error starting server {e}");
        }
    });
    Ok(true)
}

#[tauri::command] 
async fn join_server(addr: String) -> Result<bool, errors::ApplicationError>  {
    let new_str = addr.clone();
    tokio::spawn(async move {
        if let Err(_) = client::join_server(&new_str).await {
            eprintln!("Error");
        } else {
            // println!("What's going on?");
        }
    });
    Ok(true)
}

#[tauri::command]
async fn leave_chat_room() -> Result<(), String> {
    Ok(())
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![dumb, start_server, join_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
