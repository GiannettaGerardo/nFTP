mod server;

use crate::server::parser::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello from nFTP server!"); 

    let server_parser = Parser::new();

    Ok(())
}