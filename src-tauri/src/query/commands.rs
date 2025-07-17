use crate::{
    config::get_data_path,
    query::{
        audio::{AudioItem, AudioListItem},
        bookmark::Bookmark,
        bookmark_dictation::BookmarkDictationView,
        dictation::Dictation,
        setting::{AppSettings, UpdateSettingsRequest},
    },
    DbState,
};
use std::io::ErrorKind;
use tauri::AppHandle;
use tokio::fs::remove_dir_all;

use super::{
    audio::{create_audio, delete_audio, get_audio, get_audios, update_audio_transcribe},
    bookmark::{create_bookmark_item, delete_bookmark_item},
    bookmark_dictation::get_bookmark_dictation_combined,
    dictation::{create_dictation_item, delete_dictation_item, get_dictation_list},
    oauth::handle_google_auth,
    setting::{get_or_create_settings, update_app_settings},
    store::{delete_store_token, get_store_token, set_store_token},
    user::{delete_session, get_user_by_session_token, update_user_name, SessionWithUser, Timestamp},
};

async fn remove_dir_all_safe(path: &str) -> tokio::io::Result<()> {
    match remove_dir_all(path).await {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()), // File doesn't exist, that's fine
        Err(e) => Err(e),                                    // Some other error occurred
    }
}

#[derive(serde::Deserialize, specta::Type)]
pub struct TokenData {
    pub access_token: Option<String>,
    pub access_token_expires_at: Option<Timestamp>,
    pub refresh_token: Option<String>,
    pub refresh_token_expires_at: Option<Timestamp>,
}

#[tauri::command]
#[specta::specta]
pub async fn handle_login(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    sub: String,
    email: String,
    name: String,
    picture: Option<String>,
    email_verified: bool,
    tokens: TokenData,
) -> Result<String, String> {
    let db = &state.db;

    let session_token = handle_google_auth(
        &db,
        sub,
        email,
        name,
        picture,
        email_verified,
        tokens.access_token,
        tokens.access_token_expires_at,
        tokens.refresh_token,
        tokens.refresh_token_expires_at,
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

#[tauri::command]
#[specta::specta]
pub async fn check_persist_user(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
) -> Result<Option<SessionWithUser>, String> {
    let db = &state.db;
    let session_token = get_store_token(&app_handle)?;

    let user_info = get_user_by_session_token(db, &app_handle, session_token)
        .await
        .expect("Failed to get user");

    Ok(user_info)
}

#[tauri::command]
#[specta::specta]
pub async fn logout_user(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let db = &state.db;
    let session_token = delete_store_token(&app_handle)?;

    if let Some(token) = session_token {
        let _ = delete_session(db, token);
    }

    Ok(())
}

#[derive(serde::Deserialize, specta::Type)]
pub struct CreateAudioData {
    pub audio_id: String,
    pub token: String,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub thumbnail: String,
    pub start_time: i16,
    pub end_time: i16,
    pub provider: String,
    pub tag: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn handle_create_audio(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    audio_data: CreateAudioData,
) -> Result<(), String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, audio_data.token)
        .await
        .expect("create audio failed: invalid user");

    if let Some(user) = user_info {
        create_audio(
            db,
            user.user_id,
            audio_data.audio_id,
            audio_data.title,
            audio_data.description,
            audio_data.url,
            audio_data.thumbnail,
            audio_data.start_time,
            audio_data.end_time,
            audio_data.provider,
            audio_data.tag,
        )
        .await
        .expect("create audio failed: invalid paramsters");
        return Ok(());
    } else {
        return Err("Failed to create auido".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_get_audio_list(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
) -> Result<Vec<AudioListItem>, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("get audio list failed: invalid user");

    if let Some(user) = user_info {
        let audio_list = get_audios(db, user.user_id)
            .await
            .expect("get audio list failed: invalid paramsters");
        return Ok(audio_list);
    } else {
        return Err("Failed to get auido list".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_get_audio_item(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    audio_id: String,
) -> Result<AudioItem, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("get audio item failed: invalid user");

    if let Some(user) = user_info {
        let audio_item = get_audio(db, user.user_id, audio_id)
            .await
            .expect("get audio item failed: invalid paramsters");
        return Ok(audio_item);
    } else {
        return Err("Failed to get auido item".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_update_audio_transcribe(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    audio_id: String,
) -> Result<AudioItem, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("update audio item failed: invalid user");

    if let Some(user) = user_info {
        let audio_item = update_audio_transcribe(db, user.user_id, audio_id)
            .await
            .expect("update audio item failed: invalid paramsters");
        return Ok(audio_item);
    } else {
        return Err("Failed to update auido item".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_delete_audio(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    audio_id: String,
) -> Result<Vec<AudioListItem>, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("delete audio item failed: invalid user");

    if let Some(user) = user_info {
        let data_path = get_data_path(&app_handle).unwrap_or(format!("/data/"));
        let check_dir = &format!("{}/{}", data_path, audio_id);
        remove_dir_all_safe(check_dir).await.unwrap();

        let audio_list = delete_audio(db, user.user_id, audio_id)
            .await
            .expect("delete audio failed: invalid paramsters");
        return Ok(audio_list);
    } else {
        return Err("Failed to delete auido item".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_create_bookmark_item(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    audio_id: String,
    bookmark_id: i16,
) -> Result<Vec<BookmarkDictationView>, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("create bookmark item failed: invalid user");

    if let Some(user) = user_info {
        let _ = create_bookmark_item(db, user.user_id.clone(), audio_id.clone(), bookmark_id)
            .await
            .expect("create bookmark item failed: invalid paramsters");

        let combined_list = get_bookmark_dictation_combined(db, user.user_id, audio_id)
            .await
            .expect("get bookmark dictation combined failed: invalid paramsters");
        return Ok(combined_list);
    } else {
        return Err("Failed to create bookmark item".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_delete_bookmark_item(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    audio_id: String,
    bookmark_id: i16,
) -> Result<Vec<BookmarkDictationView>, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("delete bookmark item failed: invalid user");

    if let Some(user) = user_info {
        let _ = delete_bookmark_item(db, user.user_id.clone(), audio_id.clone(), bookmark_id)
            .await
            .expect("delete bookmark item failed: invalid paramsters");

        let combined_list = get_bookmark_dictation_combined(db, user.user_id, audio_id)
            .await
            .expect("get bookmark dictation combined failed: invalid paramsters");
        return Ok(combined_list);
    } else {
        return Err("Failed to delete bookmark item".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_create_dictation_item(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    audio_id: String,
    dictation_id: i16,
) -> Result<Vec<BookmarkDictationView>, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("create dictation item failed: invalid user");

    if let Some(user) = user_info {
        let _ = create_dictation_item(db, user.user_id.clone(), audio_id.clone(), dictation_id)
            .await
            .expect("create dictation item failed: invalid paramsters");

        let combined_list = get_bookmark_dictation_combined(db, user.user_id, audio_id)
            .await
            .expect("get bookmark dictation combined failed: invalid paramsters");
        return Ok(combined_list);
    } else {
        return Err("Failed to create dictation item".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_delete_dictation_item(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    audio_id: String,
    dictation_id: i16,
) -> Result<Vec<BookmarkDictationView>, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("delete dictation item failed: invalid user");

    if let Some(user) = user_info {
        let _ = delete_dictation_item(db, user.user_id.clone(), audio_id.clone(), dictation_id)
            .await
            .expect("delete dictation item failed: invalid paramsters");

        let combined_list = get_bookmark_dictation_combined(db, user.user_id, audio_id)
            .await
            .expect("get bookmark dictation combined failed: invalid paramsters");
        return Ok(combined_list);
    } else {
        return Err("Failed to delete dictation item".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_get_bookmark_dictation_combined(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    audio_id: String,
) -> Result<Vec<BookmarkDictationView>, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("get bookmark dictation combined failed: invalid user");

    if let Some(user) = user_info {
        let combined_list = get_bookmark_dictation_combined(db, user.user_id, audio_id)
            .await
            .expect("get bookmark dictation combined failed: invalid paramsters");
        return Ok(combined_list);
    } else {
        return Err("Failed to get bookmark dictation combined".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_get_app_settings(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
) -> Result<AppSettings, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("get app settings failed: invalid user");

    if let Some(_user) = user_info {
        match get_or_create_settings(db).await {
            Ok(settings) => Ok(settings),
            Err(e) => Err(format!("Failed to get app settings: {}", e)),
        }
    } else {
        return Err("Failed to get app settings".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_update_app_settings(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    request: UpdateSettingsRequest,
) -> Result<AppSettings, String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("update app settings failed: invalid user");

    if let Some(_user) = user_info {
        match update_app_settings(db, request).await {
            Ok(settings) => Ok(settings),
            Err(e) => Err(format!("Failed to update app settings: {}", e)),
        }
    } else {
        return Err("Failed to update app settings".to_string());
    }
}

#[tauri::command]
#[specta::specta]
pub async fn handle_update_user_name(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    new_name: String,
) -> Result<(), String> {
    let db = &state.db;

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .expect("update user name failed: invalid user");

    if let Some(user) = user_info {
        match update_user_name(db, &user.user_id, new_name).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to update user name: {}", e)),
        }
    } else {
        return Err("Failed to update user name".to_string());
    }
}
