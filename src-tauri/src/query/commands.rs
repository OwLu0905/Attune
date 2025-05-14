use tauri::AppHandle;

use crate::DbState;

use super::{
    oauth::handle_google_auth,
    store::{get_session_token, store_session_token},
    user::{get_user_by_session_token, SessionWithUser, Timestamp},
};

#[tauri::command(rename_all = "snake_case")]
pub async fn handle_login(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    sub: &str,
    email: &str,
    name: &str,
    picture: Option<&str>,
    email_verified: bool,
    access_token: Option<&str>,
    access_token_expires_at: Option<Timestamp>,
    refresh_token: Option<&str>,
    refresh_token_expires_at: Option<Timestamp>,
) -> Result<String, String> {
    let db = &state.db;

    let session_token = handle_google_auth(
        &db,
        sub,
        email,
        name,
        picture,
        email_verified,
        access_token,
        access_token_expires_at,
        refresh_token,
        refresh_token_expires_at,
    )
    .await
    .map_err(|e| e.to_string());

    match session_token {
        Ok(token) => {
            let _ = store_session_token(app_handle, &token).expect("Failed to store session token");
            Ok(token)
        }
        Err(er) => Err(er),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn check_persist_user(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
) -> Result<Option<SessionWithUser>, String> {
    let db = &state.db;
    let session_token = get_session_token(&app_handle)?;

    let user_info = get_user_by_session_token(db, &session_token)
        .await
        .expect("Failed to get user");

    Ok(user_info)
}
