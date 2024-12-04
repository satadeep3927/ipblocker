use tokio::{fs, sync::OnceCell};

use types::Config;

pub mod types;

static APP_CONFIG: OnceCell<Config> = OnceCell::const_new();

pub async fn read_config(path: String) {
    let json = fs::read_to_string(path)
        .await
        .expect("Error: Failed to open config file");

    let config = serde_json::from_str::<Config>(&json).expect("Error: failed to parse config file");

    APP_CONFIG
        .set(config)
        .expect("Error: Failed tp create config singleton");
}

pub async fn get_config() -> Config {
    return APP_CONFIG
        .get()
        .expect("Error: Config Singleton not initiated")
        .clone();
}
