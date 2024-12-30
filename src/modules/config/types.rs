use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Config {
    pub name: String,
    pub server: Server,
    pub rules: Vec<Rule>,
    pub database: String,
    pub abuseip: AbuseIP,
    pub whitelists: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AbuseIP {
    pub token: Vec<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub conf: Conf,
    pub log: Log,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Conf {
    pub location: String,
    pub template: String,
    pub reload: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub location: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Rule {
    RateLimitRule {
        name: String,
        path: String,
        requests: u32,
        window: u32,
    },
    AbuseReportRule {
        name: String,
        path: String,
        confidence: u32,
        delta: i64,
    },
}
