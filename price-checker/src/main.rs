use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::query::*;
mod query;


async fn query_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let (request, _context) = event.into_parts();
    let symbol = request.symbol;
    let message = query_price(symbol)?;
    Ok(Response { message })
}

#[derive(Serialize, Deserialize)]
struct Request {
    symbol: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}


#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let query = service_fn(query_handler);
    run(query).await
}
