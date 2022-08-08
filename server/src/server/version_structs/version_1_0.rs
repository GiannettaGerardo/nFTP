use std::path::PathBuf;
use async_trait::async_trait;
use tokio::{net::TcpStream, io::AsyncWriteExt};
use crate::server::{
    version_trait::{
        Version, 
        Istruction
    }, 
    parser::{
        istruction_recognition, 
        path_recognition
    }, 
    response::{
        ResponseHeader, 
        RC_OK
    }, tree_serialization
};

pub struct Version1_0;

impl Version for Version1_0 {
    fn parse(&self,
        input_bytes: &Vec<u8>,
        total_len: &usize, 
        acc_len: &mut usize,
        index: &mut usize
    ) -> Option<Box<dyn Istruction>> 
    {
        let istruction = istruction_recognition(input_bytes, total_len, acc_len, index);
        let istruction = if istruction.is_none() { return None } else { istruction.unwrap() };

        let paths = path_recognition(input_bytes, total_len, acc_len, index);
        let paths = if paths.is_none() { return None } else { paths.unwrap() };

        match istruction {
            0 => return Some(Box::new(Get {paths})),
            1 => return Some(Box::new(List)),
            // 2 => return Some(Box::new(Insert)),

            _ => {
                println!("Bad Istruction");
                return None;
            }
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
    async fn execute(&self, socket: &mut TcpStream, _: &Vec<u8>, main_path: &PathBuf) -> bool {
        if self.paths.len() != 1 {
            println!("Too many paths for GET request");
            return false;
        }

        let complete_path = main_path.join(&self.paths[0]);
        // first syscall - check the existence of the path and if the path is a file
        if !complete_path.is_file() {
            println!("A path doesn't exists for GET request");
            return false;
        }
        
        // second syscall - get the dimension of the file (in bytes)
        let payload_dim = match complete_path.metadata() {
            Ok(n) => n.len(),
            Err(e) => {
                println!("{}", e);
                return false;
            }
        };

        let response_header = ResponseHeader::new(1, 0, RC_OK, Some(payload_dim));

        match socket.write_all(&response_header.get_header()).await {
            // third syscall - read the file in bytes
            Ok(_) => if let Err(e) = socket.write_all(&std::fs::read(complete_path).unwrap()).await {
                println!("{}", e);
                return false;
            },
            Err(e) => {
                println!("Header writing problem occurs: {}", e);
                return false;
            }
        };
        true
    }

    #[inline]
    fn get_istruction_code(&self) -> u8 {
        0u8
    }
}


pub struct List;
#[async_trait]
impl Istruction for List {
    #[inline]
    async fn execute(&self, socket: &mut TcpStream, _: &Vec<u8>, main_path: &PathBuf) -> bool {
        let mut list = String::with_capacity(1000);
        tree_serialization(main_path, &mut list);

        let response_header = ResponseHeader::new(1, 0, RC_OK, Some(list.len() as u64));
        
        match socket.write_all(&response_header.get_header()).await {
            Ok(_) => if let Err(e) = socket.write_all(list.as_bytes()).await {
                println!("{}", e);
                return false;
            },
            Err(e) => {
                println!("Header writing problem occurs: {}", e);
                return false;
            }
        };
        true
    }

    #[inline]
    fn get_istruction_code(&self) -> u8 {
        1u8
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
        
        input_bytes.push(get_code);
        input_bytes.push(2u8); // 2 paths
        
        input_bytes.extend_from_slice(&(path1.len() as u16).to_be_bytes());
        input_bytes.extend_from_slice(path1);

        input_bytes.extend_from_slice(&(path2.len() as u16).to_be_bytes());
        input_bytes.extend_from_slice(path2);

        let res = Version1_0.parse(&input_bytes, &input_bytes.len(), &mut acc_len, &mut index);
        assert!(res.is_some());
        assert_eq!(get_code, res.unwrap().get_istruction_code());
    }

    #[test]
    fn parse_should_return_list() {
        let mut input_bytes: Vec<u8> = Vec::new();
        let mut acc_len: usize = 0;
        let mut index: usize = 0;
        let list_code = 1u8;
        
        input_bytes.push(list_code);
        input_bytes.push(0u8); // 0 paths

        let res = Version1_0.parse(&input_bytes, &input_bytes.len(), &mut acc_len, &mut index);
        assert!(res.is_some());
        assert_eq!(list_code, res.unwrap().get_istruction_code());
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
        assert!(res.is_none());
    }
}