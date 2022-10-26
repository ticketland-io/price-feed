use std::sync::Arc;
use tokio::sync::Mutex;
use ticketland_core::{
  services::redis::Redis,
};
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub redis: Arc<Mutex<Redis>>,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();
    let redis = Arc::new(Mutex::new(Redis::new(&config.redis_host, &config.redis_password).await.unwrap()));

    Self {
      config,
      redis,
    }
  }
}
