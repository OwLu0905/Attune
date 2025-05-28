use crate::{
    config::get_data_path,
    query::audio::{AudioItem, AudioListItem},
    DbState,
};
use std::io::ErrorKind;
use tauri::AppHandle;
use tokio::fs::remove_dir_all;

use super::{
    audio::{create_audio, delete_audio, get_audio, get_audios, update_audio_transcribe},
    oauth::handle_google_auth,
    store::{delete_store_token, get_store_token, set_store_token},
    user::{delete_session, get_user_by_session_token, SessionWithUser, Timestamp},
};

async fn remove_dir_all_safe(path: &str) -> tokio::io::Result<()> {
    match remove_dir_all(path).await {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()), // File doesn't exist, that's fine
        Err(e) => Err(e),                                    // Some other error occurred
    }
}

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
            let _ = set_store_token(app_handle, &token).expect("Failed to store session token");
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
    let session_token = get_store_token(&app_handle)?;

    let user_info = get_user_by_session_token(db, &app_handle, &session_token)
        .await
        .expect("Failed to get user");

    Ok(user_info)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn logout_user(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let db = &state.db;
    let session_token = delete_store_token(&app_handle)?;

    if let Some(token) = session_token {
        let _ = delete_session(db, &token);
    }

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn handle_create_audio(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    audio_id: &str,
    token: &str,
    title: &str,
    description: Option<&str>,
    url: &str,
    thumbnail: &str,
    start_time: i16,
    end_time: i16,
    provider: &str,
    tag: Option<&str>,
) -> Result<(), String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("create audio failed: invalid user");

    if let Some(user) = user_info {
        create_audio(
            db,
            &user.user_id,
            &audio_id,
            title,
            description,
            url,
            thumbnail,
            start_time,
            end_time,
            provider,
            tag,
        )
        .await
        .expect("create audio failed: invalid paramsters");
        return Ok(());
    } else {
        return Err("Failed to create auido".to_string());
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn handle_get_audio_list(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: &str,
) -> Result<Vec<AudioListItem>, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("get audio list failed: invalid user");

    if let Some(user) = user_info {
        let audio_list = get_audios(db, &user.user_id)
            .await
            .expect("get audio list failed: invalid paramsters");
        return Ok(audio_list);
    } else {
        return Err("Failed to get auido list".to_string());
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn handle_get_audio_item(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: &str,
    audio_id: &str,
) -> Result<AudioItem, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("get audio item failed: invalid user");

    if let Some(user) = user_info {
        let audio_item = get_audio(db, &user.user_id, audio_id)
            .await
            .expect("get audio item failed: invalid paramsters");
        return Ok(audio_item);
    } else {
        return Err("Failed to get auido item".to_string());
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn handle_update_audio_transcribe(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: &str,
    audio_id: &str,
) -> Result<AudioItem, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("update audio item failed: invalid user");

    if let Some(user) = user_info {
        let audio_item = update_audio_transcribe(db, &user.user_id, audio_id)
            .await
            .expect("update audio item failed: invalid paramsters");
        return Ok(audio_item);
    } else {
        return Err("Failed to update auido item".to_string());
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn handle_delete_audio(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: &str,
    audio_id: &str,
) -> Result<Vec<AudioListItem>, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("delete audio item failed: invalid user");

    if let Some(user) = user_info {
        let data_path = get_data_path(&app_handle).unwrap_or(format!("/data/"));
        let check_dir = &format!("{}/{}", data_path, audio_id);
        remove_dir_all_safe(check_dir).await.unwrap();

        let audio_list = delete_audio(db, &user.user_id, audio_id)
            .await
            .expect("delete audio failed: invalid paramsters");
        return Ok(audio_list);
    } else {
        return Err("Failed to delete auido item".to_string());
    }
}
