use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::db::Db;

#[derive(Debug, Serialize, Deserialize, FromRow, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub id: i64,
    #[sqlx(rename = "currentUserId")]
    pub current_user_id: Option<String>,
    pub theme: String,
    pub language: String,
    #[sqlx(rename = "selectedModel")]
    pub selected_model: String,
    #[sqlx(rename = "modelProxy")]
    pub model_proxy: Option<String>,
    #[sqlx(rename = "lastLogin")]
    pub last_login: Option<String>,
    #[sqlx(rename = "autoLogin")]
    pub auto_login: bool,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsRequest {
    pub theme: Option<String>,
    pub language: Option<String>,
    pub selected_model: Option<String>,
    pub model_proxy: Option<String>,
    pub auto_login: Option<bool>,
}

pub async fn get_app_settings(db: &Db) -> Result<AppSettings, sqlx::Error> {
    let settings = sqlx::query_as::<_, AppSettings>(
        "SELECT id, currentUserId, theme, language, selectedModel, modelProxy, lastLogin, autoLogin FROM app_settings LIMIT 1"
    )
    .fetch_one(db)
    .await?;

    Ok(settings)
}

pub async fn update_app_settings(
    db: &Db,
    request: UpdateSettingsRequest,
) -> Result<AppSettings, sqlx::Error> {
    let mut query_parts = Vec::new();
    let mut bind_values = Vec::new();

    if let Some(theme) = &request.theme {
        query_parts.push("theme = ?");
        bind_values.push(theme.as_str());
    }

    if let Some(language) = &request.language {
        query_parts.push("language = ?");
        bind_values.push(language.as_str());
    }

    if let Some(selected_model) = &request.selected_model {
        query_parts.push("selectedModel = ?");
        bind_values.push(selected_model.as_str());
    }

    if let Some(model_proxy) = &request.model_proxy {
        query_parts.push("modelProxy = ?");
        bind_values.push(model_proxy.as_str());
    }

    if let Some(auto_login) = request.auto_login {
        query_parts.push("autoLogin = ?");
        bind_values.push(if auto_login { "1" } else { "0" });
    }

    if query_parts.is_empty() {
        return get_app_settings(db).await;
    }

    let query = format!(
        "UPDATE app_settings SET {} WHERE id = (SELECT id FROM app_settings LIMIT 1)",
        query_parts.join(", ")
    );

    let mut query_builder = sqlx::query(&query);
    for value in bind_values {
        query_builder = query_builder.bind(value);
    }

    query_builder.execute(db).await?;

    get_app_settings(db).await
}

pub async fn create_default_settings(db: &Db) -> Result<AppSettings, sqlx::Error> {
    sqlx::query(
        "INSERT INTO app_settings (theme, language, selectedModel, modelProxy, autoLogin) VALUES (?, ?, ?, ?, ?)"
    )
    .bind("light")
    .bind("en")
    .bind("base.en")
    .bind(None::<String>)
    .bind(false)
    .execute(db)
    .await?;

    get_app_settings(db).await
}

pub async fn get_or_create_settings(db: &Db) -> Result<AppSettings, sqlx::Error> {
    match get_app_settings(db).await {
        Ok(settings) => Ok(settings),
        Err(sqlx::Error::RowNotFound) => create_default_settings(db).await,
        Err(e) => Err(e),
    }
}