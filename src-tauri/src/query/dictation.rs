use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::db::Db;

#[derive(Debug, Serialize, Deserialize, FromRow, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Dictation {
    #[sqlx(rename = "dictationId")]
    pub dictation_id: i16,

    #[sqlx(rename = "createdAt")]
    created_at: String,
}

pub async fn create_dictation_item(
    db: &Db,
    user_id: String,
    audio_id: String,
    dictation_id: i16,
) -> Result<Vec<Dictation>, sqlx::Error> {
    sqlx::query("INSERT INTO dictation (dictationId, userId, audioId, createdAt) VALUES (?, ?, ?, CURRENT_TIMESTAMP)")
        .bind(dictation_id)
        .bind(&user_id)
        .bind(&audio_id)
        .execute(db)
        .await?;

    let dictation_list = get_dictation_list(db, user_id, audio_id).await?;
    Ok(dictation_list)
}
pub async fn get_dictation_list(
    db: &Db,
    user_id: String,
    audio_id: String,
) -> Result<Vec<Dictation>, sqlx::Error> {
    let dictation_list = sqlx::query_as::<_, Dictation>("SELECT dictationId, createdAt FROM dictation WHERE userId = ? AND audioId = ? ORDER BY dictationId ASC")
        .bind(&user_id)
        .bind(&audio_id)
        .fetch_all(db)
        .await.map_err(|e| e.to_string());

    Ok(dictation_list.unwrap())
}

pub async fn delete_dictation_item(
    db: &Db,
    user_id: String,
    audio_id: String,
    dictation_id: i16,
) -> Result<Vec<Dictation>, sqlx::Error> {
    sqlx::query("DELETE FROM dictation WHERE dictationId = ? AND userId = ? AND audioId = ?")
        .bind(dictation_id)
        .bind(&user_id)
        .bind(&audio_id)
        .execute(db)
        .await?;

    let dictation_list = get_dictation_list(db, user_id, audio_id).await?;
    Ok(dictation_list)
}
