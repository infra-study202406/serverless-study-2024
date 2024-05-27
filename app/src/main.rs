use lambda_runtime::{service_fn, Error};

use app::handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler::lamdba_handler)).await
}

async fn my_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    println!("Hello, world!!");
    Ok(json!({ "message": "Hello, world!!", "input": event }))
}