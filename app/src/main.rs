use lambda_runtime::{
    service_fn, 
    LambdaEvent, 
    Error};
//use serde::{Deserialize, Serialize};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error>{
    println!("Hello, world!!");
    Ok(json!({ "message": "Hello, world!!", "input": event }))
}
