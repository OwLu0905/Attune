use serde_json::json;
use tauri::{App, AppHandle};
use tauri_plugin_store::StoreExt;

pub fn store_session_token(app_handle: AppHandle, session_token: &str) -> Result<(), String> {
    let store = app_handle
        .store("cookie.json")
        .expect("Failed to open store");

    store.set(
        "session_token",
        json!({
            "session_token": session_token
        }),
    );

    // store.close_resource();

    Ok(())
}
