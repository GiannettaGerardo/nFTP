mod server;

use crate::server::parser::process_request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, server!"); 

    Ok(())
}