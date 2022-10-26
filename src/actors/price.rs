use std::sync::Arc;
use actix::prelude::*;
use core::time::Duration;
use crate::utils::store::Store;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Start;

pub struct PriceActor {
  store: Arc<Store>,
  poll_interval: Duration
}

impl PriceActor {
  pub fn new(store: Arc<Store>, poll_interval: Duration) -> Self {
    Self {
      store,
      poll_interval
    }
  }
}

impl Actor for PriceActor {
  type Context = Context<Self>;
}

impl Handler<Start> for PriceActor {
  type Result = ResponseActFuture<Self, ()>;

  fn handle(&mut self, msg: Start, _: &mut Self::Context) -> Self::Result {
    let fut = async move {
      
    }
    .into_actor(self)
    .map(|_, slf, ctx| {
      ctx.notify_later(msg, slf.poll_interval);
    });

    Box::pin(fut)
  }
}
