use std::env;

pub struct Config {
  pub redis_host: String,
  pub redis_password: String,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Ok(
      Self {
        redis_host: env::var("REDIS_HOST").unwrap(),
        redis_password: env::var("REDIS_PASSWORD").unwrap(),
      }
    )
  }
}
