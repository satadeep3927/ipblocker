pub mod types;

use std::{path::PathBuf, process::Command};

use chrono::Local;
use minijinja::{context, Environment};
use sqlx::SqlitePool;
use tokio::fs::{self, read_to_string};
use types::Entity;

use super::{
    abuse_module::types::HistoryIP,
    config::{self, types::Config},
    database,
};

pub struct Blocker {
    config: Config,
    pool: SqlitePool,
}

impl Blocker {
    pub async fn new() -> Self {
        let config = config::get_config().await;
        let pool = database::get_database().await;

        return Blocker { config, pool };
    }

    pub async fn block_ip(&self, ip: &str, reason: &str) {
        let history = sqlx::query_as::<_, HistoryIP>("SELECT * FROM logs WHERE ip = $1")
            .bind(ip)
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default();

        if history.len() > 0 {
            println!("❗{} ALREADY ADDED TO DATABASE", ip);
            return;
        }

        if self.config.whitelists.contains(&ip.to_owned()) {
            println!("❗{} FOUND IN WHITELIST", ip);
        }

        sqlx::query("INSERT INTO logs (ip, reason) VALUES ($1, $2)")
            .bind(ip)
            .bind(reason)
            .execute(&self.pool)
            .await
            .ok();

        println!("✅ IP ADDED TO BLOCK LIST");

        return;
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
                "Apache Reloaded:\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        } else {
            eprintln!(
                "Apache Reload Failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    pub async fn sync_latest(&self) {
        let records = sqlx::query_as::<_, Entity>(
            r#"SELECT *
                     FROM logs
                    WHERE strftime('%Y-%m', timestamp) = strftime('%Y-%m', 'now')"#,
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let conf = &self.render_template(records).await;
        let month = Local::now().format("%m").to_string();

        let conf_path: &PathBuf = &self.resolve_conf(&month).into();

        if conf_path.parent().is_some() && !conf_path.parent().unwrap().exists() {
            fs::create_dir_all(conf_path.parent().unwrap()).await.ok();
        }

        fs::write(conf_path, conf).await.ok();
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
}
