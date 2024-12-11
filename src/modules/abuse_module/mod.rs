pub mod types;

use std::{collections::HashSet, sync::LazyLock};

use super::config;
use crate::constants::ABUSEDB_URL;
use chrono::Local;
use chrono::{DateTime, Duration, Utc};
use regex::Regex;
use sqlx::SqlitePool;
use tokio::fs::read_to_string;
use types::{AbuseIpResponse, HistoryIP};

static LOG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?P<ip>\d{1,3}(?:\.\d{1,3}){3}) - - \[(?P<timestamp>[^\]]+)]").unwrap()
});

pub async fn extract_suspects(
    name: String,
    path: String,
    confidence: u32,
    pool: &SqlitePool,
    delta: i64,
    previous_suspects: &Vec<(String, String)>,
) -> Vec<(String, String)> {
    let config = config::get_config().await;

    let whitelists = config.whitelists;

    let mut suspects: Vec<(String, String)> = vec![];

    let logfile = resolve_logfile_template(&config.server.log.location);

    let access_records =
        extract_recent_ips(&logfile, &path, delta, &config.server.log.timestamp).await;

    for ip in access_records {
        if whitelists.contains(&ip) {
            continue;
        }

        if previous_suspects.iter().any(|s| s.0 == ip) {
            continue;
        }
        let history =
            sqlx::query_as::<_, HistoryIP>("SELECT ip, reason FROM logs WHERE ip = $1 LIMIT 1")
                .bind(&ip)
                .fetch_optional(pool)
                .await
                .unwrap_or_default();

        if let Some(h) = history {
            suspects.push((ip.clone(), h.reason.clone()));
            continue;
        }

        let abusedb_confidence = get_ip_report(&ip, &config.abuseip.token).await.unwrap_or(0);

        if abusedb_confidence < confidence {
            continue;
        }

        suspects.push((
            ip.clone(),
            format!(
                "(RULE: {})[IP: {} IS A POTENTIAL SPAM (CONFIDENCE: {})]",
                name, ip, abusedb_confidence
            ),
        ));
    }

    return suspects;
}

pub fn resolve_logfile_template(template: &str) -> String {
    let now = Local::now();
    let filename = template
        .replace("{YYYY}", &now.format("%Y").to_string())
        .replace("{YY}", &now.format("%y").to_string())
        .replace("{MM}", &now.format("%m").to_string())
        .replace("{DD}", &now.format("%d").to_string())
        .replace("{MD}", &now.format("%m%d").to_string())
        .replace(
            "{M}",
            &now.format("%m")
                .to_string()
                .trim_start_matches('0')
                .to_string(),
        )
        .replace(
            "{D}",
            &now.format("%d")
                .to_string()
                .trim_start_matches('0')
                .to_string(),
        )
        .replace("{YYYYMMDD}", &now.format("%Y%m%d").to_string());

    filename
}

pub async fn extract_recent_ips(
    path: &str,
    target_route: &str,
    delta: i64,
    timestamp_fmt: &str,
) -> HashSet<String> {
    let log = read_to_string(path).await.unwrap_or_default();

    let now = Utc::now();

    log.lines()
        .filter(|line| line.contains(target_route))
        .filter_map(|line| {
            if let Some(captures) = LOG_REGEX.captures(line) {
                let ip = captures["ip"].to_string();
                let timestamp_str = captures["timestamp"].to_string();

                if let Ok(timestamp) = DateTime::parse_from_str(&timestamp_str, timestamp_fmt) {
                    let timestamp = timestamp.with_timezone(&Utc);
                    if now - timestamp <= Duration::seconds(delta) {
                        return Some(ip);
                    }
                }
            }
            None
        })
        .collect()
}

pub async fn get_ip_report(ip: &str, token: &str) -> Option<u32> {
    let url: String = format!("{}?ipAddress={}", ABUSEDB_URL, ip);

    let client = reqwest::Client::new();

    let response = match client.get(url).header("key", token).send().await {
        Ok(r) => r,
        Err(er) => {
            dbg!(er);
            return None;
        }
    };

    let payload = match response.json::<AbuseIpResponse>().await {
        Ok(p) => p,
        Err(_) => return None,
    };

    return Some(payload.data.abuse_confidence_score);
}
