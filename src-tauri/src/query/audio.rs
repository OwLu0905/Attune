use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::db::Db;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub id: String,

    #[sqlx(rename = "userId")]
    pub user_id: String,

    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub thumbnail: Option<String>,

    #[sqlx(rename = "startTime")]
    pub start_time: i16,

    #[sqlx(rename = "endTime")]
    pub end_time: i16,

    pub provider: String,
    pub tag: Option<String>,

    #[sqlx(rename = "lastUsedAt")]
    last_used_at: String,

    #[sqlx(rename = "createdAt")]
    created_at: String,

    #[sqlx(rename = "updatedAt")]
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AudioListItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub thumbnail: Option<String>,
    #[sqlx(rename = "startTime")]
    pub start_time: i16,
    #[sqlx(rename = "endTime")]
    pub end_time: i16,
    pub provider: String,
    pub tag: Option<String>,
    pub transcribe: i16,
    #[sqlx(rename = "lastUsedAt")]
    last_used_at: String,
    // #[sqlx(rename = "createdAt")]
    // created_at: String,
    // #[sqlx(rename = "updatedAt")]
    // updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AudioExerciseTypes {
    #[sqlx(rename = "audioId")]
    pub audio_id: String,

    #[sqlx(rename = "exerciseType")]
    exercise_type: String,
}

pub async fn create_audio(
    db: &Db,
    user_id: &str,
    title: &str,
    description: Option<&str>,
    url: &str,
    thumbnail: &str,
    start_time: i16,
    end_time: i16,
    provider: &str,
    tag: Option<&str>,
) -> Result<String, sqlx::Error> {
    let audio_id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO audio (
            id,
            userId,
            title,
            description,
            url,
            thumbnail,
            startTime,
            endTime,
            provider,
            lastUsedAt,
            createdAt,
            updatedAt
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
        )
        "#,
    )
    .bind(&audio_id)
    .bind(&user_id)
    .bind(title)
    .bind(description)
    .bind(url)
    .bind(thumbnail)
    .bind(start_time)
    .bind(end_time)
    .bind(provider)
    .execute(db)
    .await?;

    Ok(audio_id)
}

pub async fn get_audios(db: &Db, user_id: &str) -> Result<Vec<AudioListItem>, sqlx::Error> {
    let audios = sqlx::query_as::<_, AudioListItem>("SELECT id, title, description, url, thumbnail, startTime, endTime, provider, tag, transcribe, lastUsedAt FROM audio WHERE userId = ?  ORDER BY lastUsedAt DESC")
        .bind(user_id)
        .fetch_all(db)
        .await?;

    Ok(audios)
}
