use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{App, Manager};

pub type Db = Pool<Sqlite>;

pub async fn setup_db(app: &App) -> Db {
    let mut path = app.path().app_data_dir().expect("failed to get data_dir");

    // NOTE:
    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("error creating directory {}", err);
        }
    };

    path.push("db.sqlite");

    let url = format!("sqlite:{}", path.to_str().expect(""));

    Sqlite::create_database(url.as_str())
        .await
        .expect("failed to create database");

    let db = SqlitePoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .unwrap();

    // Drop and recreate schema
    // let err = sqlx::query("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
    //     .execute(&db)
    //     .await
    //     .map_err(|e| e.to_string());

    // TODO: add migrate
    let _ = sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .map_err(|e| e.to_string());

    db
}
