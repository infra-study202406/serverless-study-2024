use lambda_runtime::{
    service_fn, 
    LambdaEvent, 
    Error};
//use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error>{
    println!("Hello, world!!");
    Ok(())
}
