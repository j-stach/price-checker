use reqwest::blocking::Client;
use serde::Deserialize;
use lambda_runtime::{LambdaEvent, Error};
use serde_json::{Value, json};



#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PriceQuote {
    pub s: String,
    pub symbol: Vec<String>,
    pub ask: Vec<f32>,
    pub askSize: Vec<u32>,
    pub bid: Vec<f32>,
    pub bidSize: Vec<u32>,
    pub mid: Vec<f32>,
    pub last: Vec<f32>,
    pub volume: Vec<u32>,
    pub updated: Vec<u32>,
}

pub fn query_price(symbol: String) -> Result<String, reqwest::Error> {
    //let symbol = "AAPL";

    let url = format!("https://api.marketdata.app/v1/stocks/quotes/{}", symbol);
    let client = Client::new();
    let resp = client.get(url).send()?;
    let price_quote: PriceQuote = resp.json()?;

    let symbol: &String = &price_quote.symbol[0];
    let last_price: &f32 = &price_quote.last[0];

    Ok(format!("Last price for {} is {}, but you can't afford it broke boy", symbol, last_price).to_string())
}