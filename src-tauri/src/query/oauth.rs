use crate::db::Db;

use super::user::{
    create_account, create_session, create_user, get_user_by_id, Account, Timestamp,
};

async fn check_google_account_exists(db: &Db, sub: &str) -> Result<Option<Account>, sqlx::Error> {
    sqlx::query_as::<_, Account>(
        "SELECT * FROM account WHERE providerId = 'google' AND accountId = ?",
    )
    .bind(sub)
    .fetch_optional(db)
    .await
}

pub async fn handle_google_auth(
    db: &Db,
    sub: &str,
    email: &str,
    name: &str,
    picture: Option<&str>,
    email_verified: bool,
    access_token: Option<&str>,
    access_token_expires_at: Option<Timestamp>,
    refresh_token: Option<&str>,
    refresh_token_expires_at: Option<Timestamp>,
) -> Result<String, sqlx::Error> {
    let account_result = check_google_account_exists(db, sub).await?;

    let user_id = match account_result {
        Some(acc) => {
            let user_id = acc.user_id;
            user_id
        }
        None => {
            let user_id = create_user(db, name, email, email_verified, picture).await?;
            let _ = create_account(
                db,
                &user_id,
                sub,
                "google",
                access_token,
                access_token_expires_at,
                refresh_token,
                refresh_token_expires_at,
            )
            .await;
            user_id
        }
    };

    let user = get_user_by_id(db, &user_id).await?;
    let user = user.expect("Edge case");
    let user_id = user.id;

    let session_token = create_session(db, &user_id).await?;

    Ok(session_token)
}
