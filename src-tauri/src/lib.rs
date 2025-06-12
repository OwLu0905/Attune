use db::{setup_db, Db};
use tauri::Manager;

pub mod commands;
mod config;
mod db;
mod model;
mod query;
mod server;
pub mod ws;
mod yt;

pub struct DbState {
    db: Db,
}
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
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init());

    builder = builder
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_sql::Builder::new().build());

    builder
        .setup(|app| {
            let app_handle_db = app.handle().clone();
            //
            tauri::async_runtime::block_on(async move {
                let db = setup_db(&app).await;

                app_handle_db.manage(DbState { db });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            yt::download_yt_sections,
            model::start_transcribe,
            server::start_oauth_server,
            server::stop_oauth_server,
            query::commands::handle_login,
            query::commands::check_persist_user,
            query::commands::logout_user,
            query::commands::handle_create_audio,
            query::commands::handle_get_audio_list,
            query::commands::handle_get_audio_item,
            query::commands::handle_update_audio_transcribe,
            query::commands::handle_delete_audio,
            query::commands::handle_create_dictation_item,
            query::commands::handle_delete_dictation_item,
            query::commands::handle_get_dictation_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
