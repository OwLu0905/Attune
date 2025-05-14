use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Deserialize, Serialize, Debug)]
struct SessionTokenStore {
    session_token: String,
}

pub fn store_session_token(app_handle: AppHandle, session_token: &str) -> Result<(), String> {
    let store = app_handle
        .store("cookie.json")
        .expect("Failed to open store");

    let session_data = SessionTokenStore {
        session_token: session_token.to_string(),
    };

    let json_value = serde_json::to_value(session_data).expect("Failed to serialize session data");

    store.set("session_token", json_value);

    let _ = store.save().expect("Failed to save data");

    Ok(())
}

pub fn get_session_token(app_handle: &AppHandle) -> Result<String, String> {
    let store = app_handle
        .store("cookie.json")
        .expect("Failed to open store");

    match store.get("session_token") {
        Some(stored_value) => {
            let retrived_session: SessionTokenStore =
                serde_json::from_value(stored_value).expect("Failed to deserialize session data");

            let session_token = retrived_session.session_token;

            return Ok(session_token);
        }
        None => Err("Can't retrive session token".to_string()),
    }

    // store.close_resource();
}
