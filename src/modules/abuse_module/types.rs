use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbuseIpResponse {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub ip_address: Option<String>,
    pub is_public: Option<bool>,
    pub ip_version: Option<i64>,
    pub is_whitelisted: Option<bool>,
    pub abuse_confidence_score: u32,
    pub country_code: Option<String>,
    pub usage_type: Option<String>,
    pub isp: Option<String>,
    pub domain: Option<String>,
    pub hostnames: Option<Vec<Value>>,
    pub is_tor: Option<bool>,
    pub total_reports: Option<i64>,
    pub num_distinct_users: Option<i64>,
    pub last_reported_at: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct HistoryIP {
    pub ip: String,
    pub reason: String,
}
