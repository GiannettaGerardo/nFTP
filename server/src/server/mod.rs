use std::path::PathBuf;

use tokio::{net::TcpStream, io::AsyncReadExt};

pub mod parser;
pub mod version_trait;
pub mod version_structs;
pub mod response;

// test
pub static MAIN_PATH: &str = "/home/gg/Scrivania/rust";

/// Read bytes from the socket and return the number of bytes readed.
#[inline]
async fn read_bytes(socket: &mut TcpStream, buf: &mut Vec<u8>) -> Result<usize, String> {
    match (*socket).read(&mut buf[..]).await {
        Ok(n) if n <= 0 => return Err(String::from("Read 0 bytes")),
        Ok(n) => return Ok(n),
        Err(e) => return Err(e.to_string())
    }
}