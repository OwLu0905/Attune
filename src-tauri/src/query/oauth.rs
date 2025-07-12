use crate::db::Db;

use super::user::{
    create_account, create_session, create_user, get_user_by_id, Account, Timestamp,
};

async fn check_google_account_exists(
    db: &Db,
    sub: String,
    access_token: Option<String>,
    access_token_expires_at: Option<Timestamp>,
    refresh_token: Option<String>,
    refresh_token_expires_at: Option<Timestamp>,
) -> Result<Option<Account>, sqlx::Error> {
    let existing_account = sqlx::query_as::<_, Account>(
        "SELECT * FROM account WHERE providerId = 'google' AND accountId = ?",
    )
    .bind(&sub)
    .fetch_optional(db)
    .await?;

    if let Some(account) = &existing_account {
        let mut query_builder = sqlx::QueryBuilder::new("UPDATE account SET accessToken = ?");

        // Add refresh_token to the update if provided
        if refresh_token.is_some() {
            query_builder.push(", refreshToken = ?");
        }

        // Add expires_at to the update if provided
        if access_token_expires_at.is_some() {
            query_builder.push(", accessTokenExpiresAt  = ?");
        }

        if refresh_token_expires_at.is_some() {
            query_builder.push(", refreshTokenExpiresAt = ?");
        }

        // Add the WHERE condition
        query_builder.push(" WHERE id = ?");

        // Build the query
        let mut query = query_builder.build();

        query = query.bind(access_token);

        if let Some(refresh) = refresh_token {
            query = query.bind(refresh);
        }
        if let Some(expires) = access_token_expires_at {
            query = query.bind(expires);
        }
        if let Some(refresh_expires) = refresh_token_expires_at {
            query = query.bind(refresh_expires);
        }

        query = query.bind(&account.id);

        // Execute the update
        query.execute(db).await?;
    }

    Ok(existing_account)
}

pub async fn handle_google_auth(
    db: &Db,
    sub: String,
    email: String,
    name: String,
    picture: Option<String>,
    email_verified: bool,
    access_token: Option<String>,
    access_token_expires_at: Option<Timestamp>,
    refresh_token: Option<String>,
    refresh_token_expires_at: Option<Timestamp>,
) -> Result<String, sqlx::Error> {
    let account_result = check_google_account_exists(
        db,
        sub.clone(),
        access_token.clone(),
        access_token_expires_at,
        refresh_token.clone(),
        refresh_token_expires_at,
    )
    .await?;

    let user_id = match account_result {
        Some(acc) => {
            let user_id = acc.user_id;
            user_id
        }
        None => {
            let user_id = create_user(db, name, email, email_verified, picture).await?;
            let _ = create_account(
                db,
                user_id.clone(),
                sub,
                String::from("google"),
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

    let session_token = create_session(db, user_id).await?;

    Ok(session_token)
}
