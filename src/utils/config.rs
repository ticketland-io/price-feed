use std::env;
use core::time::Duration;

pub struct Config {
  pub redis_host: String,
  pub redis_password: String,
  pub redis_port: u16,
  pub poll_interval: Duration,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Ok(
      Self {
        redis_host: env::var("REDIS_HOST").unwrap(),
        redis_password: env::var("REDIS_PASSWORD").unwrap(),
        redis_port: env::var("REDIS_PORT").unwrap().parse::<u16>().unwrap(),
        poll_interval: Duration::new(env::var("POLL_INTERVAL").unwrap().parse::<u64>().unwrap(), 0),
      }
    )
  }
}
