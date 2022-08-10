use std::collections::HashMap;
use std::env;
use reqwest::header::HeaderMap;
use serde_json::json;
use serde::{Deserialize, Serialize};

const API_KEY: &str = "<API_KEY>";
const API_URL: &str = "https://api.livecoinwatch.com/coins/single";

#[derive(Serialize, Deserialize)]
struct Res {
    name: String,
    symbol: String,
    rate: f32,
    delta: HashMap<String, f32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let coin = env::var("BLOCK_INSTANCE").unwrap_or(String::from("BTC"));

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("x-api-key", API_KEY.parse().unwrap());

    let data = json!({
        "currency": "USD",
        "code": coin,
        "meta": true,
    });

    let client = reqwest::Client::new();
    let resp = client.post(API_URL)
        .headers(headers)
        .body(data.to_string())
        .send()
        .await?
        .json::<Res>()
        .await?;

    if resp.rate > 100.0 {
        println!("{} {:.0}", resp.symbol, resp.rate);
        println!("{} {:.0}", resp.symbol, resp.rate);
    } else if resp.rate > 0.1 {
        println!("{} {:.2}", resp.symbol, resp.rate);
        println!("{} {:.2}", resp.symbol, resp.rate);
    } else {
        println!("{} {:.6}", resp.symbol, resp.rate);
        println!("{} {:.6}", resp.symbol, resp.rate);
    }

    let change = resp.delta.get("week").unwrap();

    if change > &1.1 {
        println!("#FF4400");
    } else if change <= &1.1 && change > &1.0 {
        println!("#FFB400");
    } else {
        println!("#3ACC37");
    }

    Ok(())
}
