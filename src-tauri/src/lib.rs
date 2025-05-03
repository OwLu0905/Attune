use tauri::Manager;

pub mod commands;
pub mod ws;
mod yt;

pub struct DbState {
    db: String,
}

pub struct WsState {
    ws_client: ws::Client,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_shell::init());

    builder = builder.setup(|app| {
        let app_handle_db = app.handle().clone();
        // let app_handle_ws = app.handle().clone();

        tauri::async_runtime::block_on(async move {
            let db = "fewfw".to_string();
            app_handle_db.manage(DbState { db })
        });

        // tauri::async_runtime::spawn(async move {
        //     let port = ws::spwan(&app_handle_ws).await;
        //     let ws_client = ws::connect(port).await;
        //     app_handle_ws.manage(WsState { ws_client })
        // });

        Ok(())
    });

    builder
        .plugin(tauri_plugin_sql::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            yt::download_yt_sections,
            commands::ws_send
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
