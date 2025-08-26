use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::db::Db;

#[derive(Debug, Serialize, Deserialize, FromRow, specta::Type)]
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

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, specta::Type)]
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
    #[sqlx(rename = "initialPrompt")]
    pub initial_prompt: Option<String>,
    #[sqlx(rename = "updatedAt")]
    updated_at: String,
}

// Audio item (same as AudioListItem for now)
pub type AudioItem = AudioListItem;


pub async fn create_audio(
    db: &Db,
    user_id: String,
    audio_id: String,
    title: String,
    description: Option<String>,
    url: String,
    thumbnail: String,
    start_time: i16,
    end_time: i16,
    provider: String,
    _tag: Option<String>,
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

pub async fn get_audios(db: &Db, user_id: String) -> Result<Vec<AudioListItem>, sqlx::Error> {
    let audios = sqlx::query_as::<_, AudioListItem>("SELECT id, title, description, url, thumbnail, startTime, endTime, provider, tag, transcribe, initialPrompt, updatedAt FROM audio WHERE userId = ?  ORDER BY updatedAt DESC")
        .bind(&user_id)
        .fetch_all(db)
        .await?;

    Ok(audios)
}

pub async fn get_audio(
    db: &Db,
    user_id: String,
    audio_id: String,
) -> Result<AudioItem, sqlx::Error> {
    let audio = sqlx::query_as::<_, AudioItem>(
        "SELECT id, title, description, url, thumbnail, startTime, endTime, provider, tag, transcribe, initialPrompt, updatedAt FROM audio WHERE userId = ? AND id = ? ORDER BY updatedAt DESC"
    )
        .bind(&user_id)
        .bind(&audio_id)
        .fetch_one(db)
        .await?;

    Ok(audio)
}

pub async fn update_audio_updated_at(
    db: &Db,
    user_id: String,
    audio_id: String,
) -> Result<AudioItem, sqlx::Error> {
    sqlx::query("UPDATE audio SET updatedAt = CURRENT_TIMESTAMP WHERE userId = ? AND id = ?")
        .bind(&user_id)
        .bind(&audio_id)
        .execute(db)
        .await?;

    let audio = get_audio(db, user_id, audio_id).await?;

    Ok(audio)
}

pub async fn update_audio_transcribe(
    db: &Db,
    user_id: String,
    audio_id: String,
) -> Result<AudioItem, sqlx::Error> {
    sqlx::query("UPDATE audio SET transcribe = 1 WHERE userId = ? AND id = ?")
        .bind(&user_id)
        .bind(&audio_id)
        .execute(db)
        .await?;

    let audio = get_audio(db, user_id, audio_id).await?;

    Ok(audio)
}

pub async fn delete_audio(
    db: &Db,
    user_id: String,
    audio_id: String,
) -> Result<Vec<AudioListItem>, sqlx::Error> {
    sqlx::query("DELETE FROM audio WHERE userId = ? AND id = ?")
        .bind(&user_id)
        .bind(&audio_id)
        .execute(db)
        .await?;

    let audios = get_audios(db, user_id).await?;
    Ok(audios)
}

pub async fn update_audio_initial_prompt(
    db: &Db,
    user_id: String,
    audio_id: String,
    initial_prompt: Option<String>,
) -> Result<AudioItem, sqlx::Error> {
    sqlx::query("UPDATE audio SET initialPrompt = ?, updatedAt = CURRENT_TIMESTAMP WHERE userId = ? AND id = ?")
        .bind(&initial_prompt)
        .bind(&user_id)
        .bind(&audio_id)
        .execute(db)
        .await?;

    let audio = get_audio(db, user_id, audio_id).await?;

    Ok(audio)
}
