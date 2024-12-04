use std::{path::PathBuf, process::Command};

use minijinja::{context, Environment};
use sqlx::SqlitePool;
use tokio::fs::{self, read_to_string};
use types::Entity;

use super::{
    config::{self, types::Config},
    database,
};

pub mod types;

pub struct Unblocker {
    config: Config,
    pool: SqlitePool,
}

impl Unblocker {
    pub async fn new() -> Self {
        let config = config::get_config().await;
        let pool = database::get_database().await;
        return Unblocker { config, pool };
    }

    pub async fn unblock_ip(&self, ip: &str) {
        let record = match sqlx::query_as::<_, Entity>("SELECT * FROM logs WHERE ip = $1 LIMIT 1")
            .bind(ip)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
        {
            Some(r) => r,
            None => {
                println!("❗{} NOT FOUND IN DATABASE", ip);
                return;
            }
        };

        sqlx::query("DELETE FROM logs WHERE id = $1")
            .bind(&record.id)
            .execute(&self.pool)
            .await
            .ok();
        self.sync_data(&record.timestamp).await;
    }

    pub fn resolve_conf(&self, month: &str) -> String {
        let filename = &self
            .config
            .server
            .conf
            .location
            .replace("{MM}", &month.trim_start_matches('0').to_string());

        filename.to_owned()
    }

    async fn render_template(&self, records: Vec<Entity>) -> String {
        let template_content = read_to_string(&self.config.server.conf.template)
            .await
            .expect("Error: Failed to read template");

        let mut env = Environment::new();

        env.add_template("conf", &template_content).ok();

        let tmpl = env.get_template("conf").unwrap();

        let conf = tmpl.render(context!(records)).unwrap();

        return conf;
    }
    pub async fn sync_data(&self, timestamp: &chrono::DateTime<chrono::Utc>) {
        let records = sqlx::query_as::<_, Entity>(
            r#"SELECT *
                     FROM logs
                    WHERE strftime('%Y-%m', timestamp) = strftime('%Y-%m', $1)"#,
        )
        .bind(timestamp)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let conf = &self.render_template(records).await;
        let month = timestamp.format("%m").to_string();

        let conf_path: &PathBuf = &self.resolve_conf(&month).into();

        if conf_path.parent().is_some() && !conf_path.parent().unwrap().exists() {
            fs::create_dir_all(conf_path.parent().unwrap()).await.ok();
        }

        fs::write(conf_path, conf).await.ok();
    }

    pub fn reload_server(&self) {
        let command = &self.config.server.conf.reload;
    
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command");
    
        if output.status.success() {
            println!(
                "✅ SERVER RELOADED:\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        } else {
            eprintln!(
                "❗ SERVER RELOAD FAILED:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
