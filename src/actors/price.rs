use std::sync::Arc;
use actix::prelude::*;
use core::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Start;

pub struct CoingeckoFetcherActor {
  store: Arc<Store>,
  token: Token,
  poll_interval: Duration
}
