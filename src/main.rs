use std::{
  sync::Arc,
  env,
};
use actix::prelude::*;
use price_feed::{
  utils::store::Store,
  actors::price::{PriceActor, Start},
};

fn main() {
  if env::var("ENV").unwrap() == "development" {
    dotenv::from_filename(".env").expect("cannot load env from a file");
  }

  let system = System::new();

  let execution = async {
    let store = Store::new().await;
    let store = Arc::new(store);

    let price = Arc::new(PriceActor::new(Arc::clone(&store)).start());
    let _ = price.send(Start).await;
  };

  let arbiter = Arbiter::new();
  arbiter.spawn(execution);
  system.run().expect("Could not run the actix-rt system");
}
