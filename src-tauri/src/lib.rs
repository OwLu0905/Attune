use tauri::Manager;

mod yt;
pub mod ws;
pub mod commands;


pub struct AppState {
    ws_client: ws::Client
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_shell::init());
    
    builder = builder
    .setup(|app| {
        let app_handle_copy = app.handle().clone();
        
        tauri::async_runtime::spawn(async move{
            let port = ws::spwan(&app_handle_copy).await;
            let ws_client = ws::connect(port).await;
            app_handle_copy.manage(AppState{
                ws_client: ws_client
            })
        });

        Ok(())
    });

        builder
        .plugin(tauri_plugin_sql::Builder::new().build())
        .invoke_handler(tauri::generate_handler![greet, yt::download_yt_sections, commands::ws_send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
