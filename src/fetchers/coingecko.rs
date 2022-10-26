use eyre::Result;
use reqwest::{Client, Response, header::{CACHE_CONTROL}};

pub async fn fetch_price(token_symbol: &str) -> Result<Response> {
  let client = Client::new();

  let price = client
  .get(format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", token_symbol))
  .header(CACHE_CONTROL, "no-cache, no-store")
  .send()
  .await?;

  Ok(price)
}
