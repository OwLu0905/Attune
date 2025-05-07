pub mod commands;
mod config;
mod model;
pub mod ws;
mod yt;

// pub struct DbState {
//     db: String,
// }

pub struct WsState {
    ws_client: ws::WebSocketClient,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app| {
            // let app_handle_db = app.handle().clone();
            // let app_handle_ws = app.handle().clone();

            // tauri::async_runtime::block_on(async move {
            //     let db = "db_state".to_string();
            //     app_handle_db.manage(DbState { db });
            // });
            //
            // tauri::async_runtime::spawn(async move {
            //     let (port, token) = ws::spwan(&app_handle_cd).await;
            //     let token_copy = token.clone();
            //     let ws_client = ws::connect(port, token_copy).await;
            //     app_handle_ws.manage(WsState { ws_client });
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            yt::download_yt_sections,
            model::start_transcribe,
            // commands::ws_send,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
