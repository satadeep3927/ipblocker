use tokio::sync::OnceCell;

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use super::config;

static DB_POOL: OnceCell<SqlitePool> = OnceCell::const_new();

pub async fn init() {
    let path = config::get_config().await.database;
    let url = format!("sqlite://{}", path);

    if !Sqlite::database_exists(&url).await.unwrap_or(false) {
        match Sqlite::create_database(&url).await {
            Ok(_) => println!("✅ DATABASE CREATED"),
            Err(error) => panic!("❌ ERROR: {}", error),
        }
    } else {
        println!("✅ DATABASE EXISTS");
    }

    let pool = SqlitePool::connect(&url)
        .await
        .expect("Error: Failed to open Database. Ensure that the path is correct");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ip TEXT NOT NULL,
            reason TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    println!("✅ DATABASE MIGRATED");

    DB_POOL
        .set(pool)
        .expect("Error: Failed to create Database pool.");
}

pub async fn get_database() -> SqlitePool {
    return DB_POOL
        .get()
        .expect("Error: failed to extract database pool")
        .clone();
}
