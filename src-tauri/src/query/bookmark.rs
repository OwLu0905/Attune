use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::db::Db;
use super::audio::update_audio_updated_at;

#[derive(Debug, Serialize, Deserialize, FromRow, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    #[sqlx(rename = "bookmarkId")]
    pub bookmark_id: i16,

    #[sqlx(rename = "createdAt")]
    created_at: String,
}

pub async fn create_bookmark_item(
    db: &Db,
    user_id: String,
    audio_id: String,
    bookmark_id: i16,
) -> Result<Vec<Bookmark>, sqlx::Error> {
    sqlx::query("INSERT INTO bookmark (bookmarkId, userId, audioId, createdAt) VALUES (?, ?, ?, CURRENT_TIMESTAMP)")
        .bind(bookmark_id)
        .bind(&user_id)
        .bind(&audio_id)
        .execute(db)
        .await?;

    // Update audio updatedAt
    let _ = update_audio_updated_at(db, user_id.clone(), audio_id.clone()).await;

    let bookmark_list = get_bookmark_list(db, user_id, audio_id).await?;
    Ok(bookmark_list)
}
pub async fn get_bookmark_list(
    db: &Db,
    user_id: String,
    audio_id: String,
) -> Result<Vec<Bookmark>, sqlx::Error> {
    let bookmark_list = sqlx::query_as::<_, Bookmark>("SELECT bookmarkId, createdAt FROM bookmark WHERE userId = ? AND audioId = ? ORDER BY bookmarkId ASC")
        .bind(&user_id)
        .bind(&audio_id)
        .fetch_all(db)
        .await.map_err(|e| e.to_string());

    Ok(bookmark_list.unwrap())
}

pub async fn delete_bookmark_item(
    db: &Db,
    user_id: String,
    audio_id: String,
    bookmark_id: i16,
) -> Result<Vec<Bookmark>, sqlx::Error> {
    sqlx::query("DELETE FROM bookmark WHERE bookmarkId = ? AND userId = ? AND audioId = ?")
        .bind(bookmark_id)
        .bind(&user_id)
        .bind(&audio_id)
        .execute(db)
        .await?;

    // Update audio updatedAt
    let _ = update_audio_updated_at(db, user_id.clone(), audio_id.clone()).await;

    let bookmark_list = get_bookmark_list(db, user_id, audio_id).await?;
    Ok(bookmark_list)
}
