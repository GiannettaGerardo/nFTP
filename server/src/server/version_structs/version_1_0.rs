use std::{path::PathBuf, error::Error};

use async_trait::async_trait;
use tokio::{net::TcpStream, io::AsyncWriteExt};

use crate::server::{version_trait::{Version, Istruction}, parser::{istruction_recognition, path_recognition}, MAIN_PATH, response::{ResponseHeader, RC_OK}};

pub struct Version1_0;

impl Version for Version1_0 {
    fn parse(&self,
        input_bytes: &Vec<u8>,
        total_len: &usize, 
        acc_len: &mut usize,
        index: &mut usize
    ) -> Result<Box<dyn Istruction>, String> 
    {
        let istruction = istruction_recognition(input_bytes, total_len, acc_len, index);
        if let Err(e) = istruction { 
            return Err(e);
        }

        let paths = path_recognition(input_bytes, total_len, acc_len, index);
        if let Err(e) = paths {
            return Err(e); 
        }
        let paths = paths.unwrap();

        match istruction.unwrap() {
            0 => return Ok(Box::new(Get {paths})),
            // 1 => return Ok(Box::new(Insert)),
            // 2 => return Ok(Box::new(List)),

            _ => return Err(String::from("Bad Istruction"))
        };
    }

    #[inline]
    fn get_version(&self) -> u8 {
        0b0001_0000u8
    }
}


/// The GET istruction
pub struct Get {
    pub paths: Vec<PathBuf>
}
#[async_trait]
impl Istruction for Get {

    /// For the GET request, execute() checks if the path exists and if it is a file,
    /// then creates a response header, writes it and the file into the socket.
    #[inline]
    async fn execute(&self, socket: &mut TcpStream, bytes: &Vec<u8>) -> Result<(), String> {
        let main_path = PathBuf::from(MAIN_PATH);

        if self.paths.len() != 1 {
            return Err(String::from("Too many paths for GET request"));
        }

        let complete_path = main_path.join(&self.paths[0]);
        // first syscall - check the existence of the path and if the path is a file
        if !complete_path.is_file() {
            return Err(String::from("A path doesn't exists for GET request"));
        }
        
        // second syscall - get the dimension of the file (in bytes)
        let payload_dim = match complete_path.metadata() {
            Ok(n) => n.len(),
            Err(e) => return Err(e.to_string())
        };

        let response_header = ResponseHeader::new(1, 0, RC_OK, Some(payload_dim));

        match socket.write(&response_header.transform()).await {
            Ok(n) if n <= 0 => return Err(String::from("")),
            // third syscall - read the file in bytes
            Ok(_) => if let Err(e) = socket.write_all(&std::fs::read(complete_path).unwrap()).await {
                return Err(e.to_string());
            },
            Err(_) => return Err(String::from(""))
        };
        Ok(())
    }

    #[inline]
    fn get_istruction_code(&self) -> u8 {
        0u8
    }
}


#[cfg(test)]
pub mod test {
    use crate::server::version_trait::*;
    use super::*;

    #[test]
    fn parse_should_return_get() {
        let mut input_bytes: Vec<u8> = Vec::new();
        let path1 = b"/dir_a/dir_b/dir_c/file.txt";
        let path2 = b"/dir_a/dir_b/dir_c/dir_d/dir_e/very_long_file.pdf";
        let mut acc_len: usize = 0;
        let mut index: usize = 0;
        let get_code = 0u8;
        
        input_bytes.push(0u8); // GET
        input_bytes.push(2u8); // 2 paths
        
        input_bytes.extend_from_slice(&(path1.len() as u16).to_be_bytes());
        input_bytes.extend_from_slice(path1);

        input_bytes.extend_from_slice(&(path2.len() as u16).to_be_bytes());
        input_bytes.extend_from_slice(path2);

        let res = Version1_0.parse(&input_bytes, &input_bytes.len(), &mut acc_len, &mut index);
        assert!(res.is_ok());
        assert_eq!(get_code, res.unwrap().get_istruction_code());
    }

    #[test]
    fn parse_with_inexistent_istruction_should_return_err() {
        let mut input_bytes: Vec<u8> = Vec::new();
        let path = b"/dir_a/dir_b/dir_c/file.txt";
        let mut acc_len: usize = 0;
        let mut index: usize = 0;
        
        input_bytes.push(100u8); // inexistent istruction
        input_bytes.push(1u8); // 1 path
        
        input_bytes.extend_from_slice(&(path.len() as u16).to_be_bytes());
        input_bytes.extend_from_slice(path);

        let res = Version1_0.parse(&input_bytes, &input_bytes.len(), &mut acc_len, &mut index);
        assert!(res.is_err());
    }
}