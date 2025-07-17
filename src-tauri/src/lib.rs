use db::{setup_db, Db};
use tauri::Manager;

mod config;
mod db;
mod model;
mod query;
mod server;
mod service;
mod yt;

use tauri_specta::{collect_commands, Builder};

pub struct DbState {
    db: Db,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init());

    let mut ts_build = Builder::<tauri::Wry>::new();
    ts_build = ts_build.commands(collect_commands![
        greet,
        yt::download_yt_sections,
        model::start_transcribe,
        model::start_transcribe_service,
        model::start_transcribe_service_streaming,
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
        query::commands::handle_create_bookmark_item,
        query::commands::handle_delete_bookmark_item,
        query::commands::handle_create_dictation_item,
        query::commands::handle_delete_dictation_item,
        query::commands::handle_get_bookmark_dictation_combined,
        query::commands::handle_get_app_settings,
        query::commands::handle_update_app_settings,
        query::commands::handle_update_user_name,
    ]);
    #[cfg(debug_assertions)] // <- Only export on non-release builds
    ts_build
        .export(
            specta_typescript::Typescript::default()
                .bigint(specta_typescript::BigIntExportBehavior::Number),
            "../src/lib/tauri.ts",
        )
        .expect("Failed to export typescript bindings");

    builder = builder
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
        .invoke_handler(ts_build.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
