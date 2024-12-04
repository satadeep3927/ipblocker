use std::collections::HashMap;

use super::config;
use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::Utc;
use regex::Regex;
use tokio::fs::read_to_string;

pub async fn extract_suspects(
    name: String,
    path: String,
    requests: u32,
    window: u32,
) -> Vec<(String, String)> {
    let config = config::get_config().await;

    let logfile = resolve_logfile_template(&config.server.log.location);

    let access_records = extract_ip_and_timestamp(&logfile, &path).await;

    let suspects = find_suspects(
        access_records,
        requests,
        window,
        &config.server.log.timestamp,
    )
    .iter()
    .filter(|ip| !config.whitelists.contains(ip))
    .map(|ip| {
        (
            ip.clone(),
            format!(
                "(RULE: {})[IP {} EXCEEDED {} REQUESTS IN {} SECONDS WINDOW]",
                name, ip, requests, window
            ),
        )
    })
    .collect();

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
pub async fn extract_ip_and_timestamp(path: &str, target_route: &str) -> Vec<(String, String)> {
    let log = read_to_string(path).await.unwrap_or_default();
    let log_regex =
        Regex::new(r"(?P<ip>\d{1,3}(?:\.\d{1,3}){3}) - - \[(?P<timestamp>[^\]]+)]").unwrap();

    log.lines()
        .filter(|line| line.contains(&target_route))
        .filter_map(|line| {
            if let Some(captures) = log_regex.captures(line) {
                Some((
                    captures["ip"].to_string(),
                    captures["timestamp"].to_string(),
                ))
            } else {
                None
            }
        })
        .collect()
}
pub fn find_suspects(
    records: Vec<(String, String)>,
    hit_count: u32,
    window_seconds: u32,
    datetime_format: &str,
) -> Vec<String> {
    let mut ip_hits: HashMap<String, Vec<DateTime<Utc>>> = HashMap::new();
    let mut result = Vec::new();

    for record in records {
        if let Ok(parsed_time) = DateTime::parse_from_str(&record.1, &datetime_format) {
            ip_hits
                .entry(record.0)
                .or_insert_with(Vec::new)
                .push(parsed_time.with_timezone(&Utc));
        }
    }

    for (ip, timestamps) in ip_hits {
        let mut sorted_timestamps = timestamps;

        sorted_timestamps.sort();

        for i in 0..sorted_timestamps.len() {
            let start_time = sorted_timestamps[i];
            let end_time = start_time + Duration::seconds(window_seconds.into());

            let hits_in_window = sorted_timestamps
                .iter()
                .filter(|&&t| t >= start_time && t <= end_time)
                .count();

            if hits_in_window >= hit_count.try_into().unwrap() {
                result.push(ip.clone());
                break;
            }
        }
    }

    result
}
