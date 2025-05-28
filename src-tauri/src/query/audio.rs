use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

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

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
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

// Audio with exercise types
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioItem {
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub audio: AudioListItem,

    #[sqlx(rename = "exerciseType")]
    pub exercise_type: Option<String>,
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
    audio_id: &str,
    title: &str,
    description: Option<&str>,
    url: &str,
    thumbnail: &str,
    start_time: i16,
    end_time: i16,
    provider: &str,
    tag: Option<&str>,
) -> Result<(), sqlx::Error> {
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

    Ok(())
}

pub async fn get_audios(db: &Db, user_id: &str) -> Result<Vec<AudioListItem>, sqlx::Error> {
    let audios = sqlx::query_as::<_, AudioListItem>("SELECT id, title, description, url, thumbnail, startTime, endTime, provider, tag, transcribe, lastUsedAt FROM audio WHERE userId = ?  ORDER BY lastUsedAt DESC")
        .bind(user_id)
        .fetch_all(db)
        .await?;

    Ok(audios)
}

pub async fn get_audio(db: &Db, user_id: &str, audio_id: &str) -> Result<AudioItem, sqlx::Error> {
    let audio = sqlx::query_as::<_, AudioItem>(
        r#"
        SELECT 
            a.id, 
            a.title, 
            a.description,
            a.url,
            a.thumbnail,
            a.startTime,
            a.endTime,
            a.provider,
            a.tag,
            a.transcribe,
            a.lastUsedAt,
            GROUP_CONCAT(aet.exerciseType) as exerciseType
        FROM audio a
        LEFT JOIN audio_exercise_types aet ON a.id = aet.audioId
        WHERE a.userId = ? AND a.id = ?  
        GROUP BY a.id, a.title, a.description, a.url, a.thumbnail, a.startTime, a.endTime, a.provider, a.tag, a.transcribe, a.lastUsedAt
        ORDER BY lastUsedAt DESC
        "#

    )
        .bind(user_id)
        .bind(audio_id)
        .fetch_one(db)
        .await.map_err(|e| e.to_string());

    Ok(audio.unwrap())
}

pub async fn update_audio_transcribe(
    db: &Db,
    user_id: &str,
    audio_id: &str,
) -> Result<AudioItem, sqlx::Error> {
    sqlx::query("UPDATE audio SET transcribe = 1 WHERE userId = ? AND id = ?")
        .bind(user_id)
        .bind(audio_id)
        .execute(db)
        .await?;

    let audio = get_audio(db, user_id, audio_id).await?;

    Ok(audio)
}

pub async fn delete_audio(
    db: &Db,
    user_id: &str,
    audio_id: &str,
) -> Result<Vec<AudioListItem>, sqlx::Error> {
    sqlx::query("DELETE FROM audio WHERE userId = ? AND id = ?")
        .bind(user_id)
        .bind(audio_id)
        .execute(db)
        .await?;

    let audios = get_audios(db, user_id).await?;
    Ok(audios)
}
