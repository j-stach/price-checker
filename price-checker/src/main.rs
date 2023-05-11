use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

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

pub async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let symbol = event["symbol"].to_string();
    let message = query_price(symbol).unwrap();
    Ok(json!({ "message": format!("{}", message) }))
}

use lambda_runtime::*;


mod query;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let func = service_fn(query::handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    command: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let command = event.payload.command;

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {}.", command),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    lambda_runtime::run(service_fn(function_handler)).await
}
