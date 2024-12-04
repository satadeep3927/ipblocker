pub mod types;

use std::collections::HashSet;

use chrono::Local;
use regex::Regex;
use sqlx::SqlitePool;
use tokio::fs::read_to_string;
use types::{AbuseIpResponse, HistoryIP};

use super::config;
use crate::constants::ABUSEDB_URL;

pub async fn extract_suspects(
    name: String,
    path: String,
    confidence: u32,
    pool: &SqlitePool,
) -> Vec<(String, String)> {
    let config = config::get_config().await;

    let whitelists = config.whitelists;

    let mut suspects: Vec<(String, String)> = vec![];

    let history = sqlx::query_as::<_, HistoryIP>("SELECT ip, reason FROM logs")
        .fetch_all(pool)
        .await
        .unwrap_or_default();
    let logfile = resolve_logfile_template(&config.server.log.location);

    let access_records = extract_ip(&logfile, &path).await;

    for ip in access_records {
        if whitelists.contains(&ip) {
            continue;
        }
        if history.iter().any(|h| h.ip == ip) {
            let h = history.iter().find(|h| h.ip == ip).unwrap();
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

pub async fn extract_ip(path: &str, target_route: &str) -> HashSet<String> {
    let log = read_to_string(path).await.unwrap_or_default();
    let log_regex =
        Regex::new(r"(?P<ip>\d{1,3}(?:\.\d{1,3}){3}) - - \[(?P<timestamp>[^\]]+)]").unwrap();

    log.lines()
        .filter(|line| line.contains(&target_route))
        .filter_map(|line| {
            if let Some(captures) = log_regex.captures(line) {
                Some(captures["ip"].to_string())
            } else {
                None
            }
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
