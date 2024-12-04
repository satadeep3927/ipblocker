pub mod types;

use prettytable::{row, Table};
use sqlx::SqlitePool;
use types::Entity;

use super::database;

#[derive(Debug)]
pub struct Shower {
    pool: SqlitePool,
}

impl Shower {
    pub async fn new() -> Self {
        let pool = database::get_database().await;
        return Shower { pool };
    }

    pub async fn dispatch(&self) {
        let records = sqlx::query_as::<_, Entity>("SELECT * FROM logs")
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default();

        let mut table = Table::new();

        table.add_row(row!["ID", "IP ADDRESS", "REASON", "TIMESTAMP"]);

        for Entity {
            id,
            ip,
            reason,
            timestamp,
        } in records
        {
            table.add_row(row![id, ip, reason, timestamp]);
        }

        table.printstd();
    }
}
