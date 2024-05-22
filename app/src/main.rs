use lambda_runtime::{ 
    service_fn,
    Error
};

use app::handler;

#[tokio::main]
async fn main() -> Result<(), Error>{
    lambda_runtime::run(service_fn(handler::lamdba_handler)).await
}