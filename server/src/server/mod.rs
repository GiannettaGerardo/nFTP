pub mod parser;
pub mod version_trait;
pub mod version_structs;
pub mod response;

use tokio::{net::TcpStream, io::AsyncReadExt};

// test
pub static MAIN_PATH: &str = "/home/gg/Scrivania/rust";

/// Read bytes from the socket and return the number of bytes readed.
#[inline]
async fn read_bytes(socket: &mut TcpStream, buf: &mut Vec<u8>) -> Option<usize> {
    match (*socket).read(&mut buf[..]).await {
        Ok(n) if n <= 0 => {
            println!("Read 0 bytes");
            return None;
        },
        Ok(n) => return Some(n),
        Err(e) => {
            println!("{}", e);
            return None;
        }
    }
}