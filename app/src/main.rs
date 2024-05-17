use lambda_runtime::{
    service_fn, 
    LambdaEvent, 
    Error};
//use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio;

// #[tokio::main]
// async fn main() -> Result<(), Error>{
//     println!("Hello, world!!");
//     Ok(json!({ "message": "Hello, world!!", "input": event }))
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    println!("Hello, world!!");
    Ok(json!({ "message": "Hello, world!!", "input": event }))
}