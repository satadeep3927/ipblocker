use super::{config, database};

pub async fn init_app(path: String) {
    config::read_config(path).await;
    database::init().await
}
