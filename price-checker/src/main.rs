use lambda_runtime::{Error, LambdaEvent};

// use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::query::*;
mod query;


pub async fn query_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let symbol = event["symbol"].to_string();
    let message = query_price(symbol)?;
    Ok(json!({ "message": format!("{}", message) }))
}


#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let func = lambda_runtime::service_fn(query_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
