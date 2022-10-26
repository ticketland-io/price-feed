use std::{
  sync::Arc,
  collections::HashMap,
};
use actix::prelude::*;
use eyre::Result;
use ticketland_core::async_helpers::with_retry;
use crate::{
  utils::store::Store,
  fetchers::coingecko,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Start;

pub struct PriceActor {
  store: Arc<Store>,
}

impl PriceActor {
  pub fn new(store: Arc<Store>) -> Self {
    Self {store}
  }

  async fn fetch_coingecko_price(token_symbol: &str) -> Result<f64> {
    let action = || {
      async {
        coingecko::fetch_price("solana").await
      }
    };

    let response = with_retry(None, Some(1), action).await?;
    let mut data: HashMap<String, HashMap<String, f64>> = response.json().await.expect("cannot deserialize coingecko response");
    let price = data
    .remove(token_symbol)
    .expect("error with coingecko data")
    .remove("usd")
    .expect("error with coingecko data");

    Ok(price)
  }
}

impl Actor for PriceActor {
  type Context = Context<Self>;
}

impl Handler<Start> for PriceActor {
  type Result = ResponseActFuture<Self, ()>;

  fn handle(&mut self, msg: Start, _: &mut Self::Context) -> Self::Result {
    let poll_interval = self.store.config.poll_interval;

    let fut = async move {
      let price = Self::fetch_coingecko_price("solana").await;
    }
    .into_actor(self)
    .map(move |_, _, ctx| {
      ctx.notify_later(msg, poll_interval);
    });

    Box::pin(fut)
  }
}
