use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    first_name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}

async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let first_name = event.payload.first_name;
    let respons = Response {
        message: format!("Hello, {first_name}!"),
    };

    Ok(respons)
}
