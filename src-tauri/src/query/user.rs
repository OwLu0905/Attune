use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tauri::AppHandle;
use uuid::Uuid;

use crate::db::Db;

use super::store::delete_store_token;

pub type Timestamp = i64;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,

    #[sqlx(rename = "emailVerified")]
    pub email_verified: bool,
    pub image: Option<String>,

    #[sqlx(rename = "createdAt")]
    created_at: String,

    #[sqlx(rename = "updatedAt")]
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,

    #[sqlx(rename = "userId")]
    pub user_id: String,

    #[sqlx(rename = "accountId")]
    pub account_id: String,

    #[sqlx(rename = "providerId")]
    pub provider_id: String,

    #[sqlx(rename = "accessToken")]
    pub access_token: Option<String>,

    #[sqlx(rename = "refreshToken")]
    pub refresh_token: Option<String>,

    #[sqlx(rename = "accessTokenExpiresAt")]
    pub access_token_expires_at: Option<Timestamp>,

    #[sqlx(rename = "refreshTokenExpiresAt")]
    pub refresh_token_expires_at: Option<Timestamp>,

    pub scope: Option<String>,

    #[sqlx(rename = "idToken")]
    pub id_token: Option<String>,

    pub password: Option<String>,

    #[sqlx(rename = "createdAt")]
    created_at: String,

    #[sqlx(rename = "updatedAt")]
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: String,

    #[sqlx(rename = "userId")]
    pub user_id: String,

    pub token: String,

    #[sqlx(rename = "expiresAt")]
    pub expires_at: Timestamp,

    #[sqlx(rename = "createdAt")]
    created_at: String,

    #[sqlx(rename = "updatedAt")]
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Verification {
    id: String,
    identifier: String,
    value: String,

    #[sqlx(rename = "expiresAt")]
    expires_at: Timestamp,

    #[sqlx(rename = "createdAt")]
    created_at: String,

    #[sqlx(rename = "updatedAt")]
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct AppSettings {
    id: i64,
    current_user_id: Option<String>,
    theme: String,
    language: String,
    last_login: Option<String>,
    auto_login: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SessionWithUser {
    pub user_id: String,
    pub access_token: String,
    name: String,
    email: String,
    picture: Option<String>,
}

pub async fn get_user_by_id(db: &Db, user_id: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?")
        .bind(user_id)
        .fetch_optional(db)
        .await
}

pub async fn get_user_by_session_token(
    db: &Db,
    app_handle: &AppHandle,
    session_token: String,
) -> Result<Option<SessionWithUser>, sqlx::Error> {
    let result = sqlx::query_as::<_, SessionWithUser>(
        r#"
        SELECT 
            u.id as "user_id", 
            s.token as "access_token", 
            u.name as "name",
            u.email as "email",
            u.image as "picture"
        FROM session s 
        JOIN user u ON s.userId = u.id
        WHERE s.token = ? AND s.expiresAt > unixepoch()
        "#,
    )
    .bind(&session_token)
    .fetch_optional(db)
    .await?;

    if result.is_none() {
        sqlx::query(
            r#"
            DELETE FROM session
            WHERE token = ?
            "#,
        )
        .bind(session_token)
        .execute(db)
        .await?;
        let _ = delete_store_token(&app_handle).unwrap();
    } else {
        sqlx::query(
            r#"
            UPDATE session
            SET expiresAt = unixepoch() + 86400
            WHERE token = ?
            "#,
        )
        .bind(session_token)
        .execute(db)
        .await?;
    }

    Ok(result)
}

pub async fn create_user(
    db: &Db,
    name: String,
    email: String,
    email_verified: bool,
    picture: Option<String>,
) -> Result<String, sqlx::Error> {
    let user_id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO user (id, name, email, emailVerified, image) VALUES (?, ?, ?, ?, ?)")
        .bind(&user_id)
        .bind(&name)
        .bind(&email)
        .bind(&email_verified)
        .bind(&picture)
        .execute(db)
        .await?;

    Ok(user_id)
}

pub async fn create_account(
    db: &Db,
    user_id: String,
    account_id: String,
    provider_id: String,
    access_token: Option<String>,
    access_token_expires_at: Option<Timestamp>,
    refresh_token: Option<String>,
    refresh_token_expires_at: Option<Timestamp>,
) -> Result<Account, sqlx::Error> {
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO account (id, userId, providerId, accountId, accessToken, accessTokenExpiresAt, refreshToken, refreshTokenExpiresAt, createdAt, updatedAt) VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
    .bind(&id)
    .bind(&user_id)
    .bind(&provider_id)
    .bind(&account_id)
    .bind(&access_token)
    .bind(&access_token_expires_at)
    .bind(&refresh_token)
    .bind(&refresh_token_expires_at)
    .execute(db)
    .await
    .expect("account error");

    let account = sqlx::query_as::<_, Account>("SELECT * FROM account WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await?;

    Ok(account)
}

pub async fn create_session(db: &Db, user_id: String) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let token = Uuid::new_v4().to_string();
    let now = Utc::now();
    let expires_at = now + Duration::seconds(86400 as i64);
    let expires_at_timestamp = expires_at.timestamp();

    sqlx::query(
        r#"
        INSERT INTO session (
            id,
            userId,
            token,
            expiresAt,
            createdAt,
            updatedAt
        ) VALUES (
            ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
        )
        "#,
    )
    .bind(&id)
    .bind(&user_id)
    .bind(&token)
    .bind(&expires_at_timestamp)
    .execute(db)
    .await?;

    Ok(token)
}

pub async fn delete_session(db: &Db, session_token: String) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM session
        WHERE token = ?
        "#,
    )
    .bind(&session_token)
    .execute(db)
    .await?;

    Ok(())
}
