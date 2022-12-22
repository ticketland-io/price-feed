use ticketland_core::{
  services::redis,
};
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub redis_pool: redis::ConnectionPool,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();
    let redis_pool = redis::ConnectionPool::new(&config.redis_host, &config.redis_password, config.redis_port);

    Self {
      config,
      redis_pool,
    }
  }
}
