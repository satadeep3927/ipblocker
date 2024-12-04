pub mod types;

use prettytable::{row, Table};
use sqlx::SqlitePool;

use super::{
    abuse_module,
    config::{
        self,
        types::{
            Config,
            Rule::{AbuseReportRule, RateLimitRule},
        },
    },
    database, rate_limit_module,
};

#[derive(Debug)]
pub struct Scanner {
    config: Config,
    pool: SqlitePool,
}

impl Scanner {
    pub async fn new() -> Self {
        let config = config::get_config().await;
        let pool = database::get_database().await;
        return Scanner { config, pool };
    }

    pub fn display_suspects(suspects: Vec<(String, String)>) {
        let mut table = Table::new();

        table.add_row(row!["IP ADDRESS", "REASON"]);

        for (ip, reason) in suspects {
            table.add_row(row![ip, reason]);
        }

        table.printstd();
    }

    pub async fn dispatch(self) -> Vec<(String, String)> {
        let rules = self.config.rules;

        let mut suspects: Vec<(String, String)> = vec![];

        for rule in rules {
            match rule {
                RateLimitRule {
                    name,
                    path,
                    requests,
                    window,
                } => {
                    let rate_limit_suspects =
                        rate_limit_module::extract_suspects(name, path, requests, window).await;

                    suspects.extend(rate_limit_suspects);
                }
                AbuseReportRule {
                    name,
                    path,
                    confidence,
                } => {
                    let abusedb_suspects =
                        abuse_module::extract_suspects(name, path, confidence, &self.pool).await;

                    suspects.extend(abusedb_suspects);
                }
            }
        }

        return suspects;
    }
}
