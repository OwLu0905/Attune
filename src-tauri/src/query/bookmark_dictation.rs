use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::db::Db;

#[derive(Debug, Serialize, Deserialize, FromRow, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkDictationView {
    #[sqlx(rename = "userId")]
    pub user_id: String,
    
    #[sqlx(rename = "audioId")]
    pub audio_id: String,
    
    #[sqlx(rename = "bookmark_id")]
    pub bookmark_id: Option<i64>,
    
    #[sqlx(rename = "bookmarkId")]
    pub bookmark_position: Option<i16>,
    
    #[sqlx(rename = "bookmark_created_at")]
    pub bookmark_created_at: Option<String>,
    
    #[sqlx(rename = "dictation_id")]
    pub dictation_id: Option<i64>,
    
    #[sqlx(rename = "dictationId")]
    pub dictation_position: Option<i16>,
    
    #[sqlx(rename = "dictation_created_at")]
    pub dictation_created_at: Option<String>,
}

pub async fn get_bookmark_dictation_combined(
    db: &Db,
    user_id: String,
    audio_id: String,
) -> Result<Vec<BookmarkDictationView>, sqlx::Error> {
    let combined_data = sqlx::query_as::<_, BookmarkDictationView>(
        "SELECT * FROM bookmark_dictation_view WHERE userId = ? AND audioId = ? ORDER BY COALESCE(bookmark_created_at, dictation_created_at) ASC"
    )
    .bind(&user_id)
    .bind(&audio_id)
    .fetch_all(db)
    .await?;

    Ok(combined_data)
}