mod server;

use std::sync::Arc;
use tokio::net::TcpListener;
use crate::server::{parser::Parser, response::{send_error_response, RC_ERROR}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello from nFTP server!"); 

    let server_parser = Arc::new(Parser::new());
    let listener = match TcpListener::bind("127.0.0.1:3000").await {
        Ok(listener) => listener,
        Err(e) => panic!("{}", e)
    };
    println!("Listening on 127.0.0.1:3000");

    loop {
        let server_parser = Arc::clone(&server_parser);
        let mut socket = match listener.accept().await {
            Ok((socket, _)) => socket,
            Err(e) => {
                println!("{}\n", e);
                continue
            }
        };

        tokio::spawn(async move {
            if !server_parser.process_request(&mut socket).await {
                send_error_response(&mut socket, RC_ERROR).await;
            }
        });
    }

    Ok(())
}