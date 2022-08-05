use std::{str::from_utf8, path::PathBuf};
use tokio::net::TcpStream;
use super::{
    version_trait::Version, 
    version_structs::version_1_0::Version1_0,
    read_bytes
};

/// Max number of accepted paths
const MAX_PATHS: u8 = 10;
/// Max dimension of the header buffer
const MAX_HEADER_BUF: usize = 1024;


/// Parse the input bytes according to nFTP protocol.
#[inline]
pub async fn process_request(socket: &mut TcpStream) {
    let mut input_bytes: Vec<u8> = vec![0; MAX_HEADER_BUF];
    
    let readed_bytes = read_bytes(socket, &mut input_bytes).await;
    let readed_bytes = if readed_bytes.is_none() { return } else { readed_bytes.unwrap() };
    println!("readed bytes {}", readed_bytes); // log

    let total_len = input_bytes.len();
    let mut acc_len: usize = 4;
    let mut index: usize = 0;
    
    if !protocol_recognition(&input_bytes, &total_len, &mut acc_len, &mut index) { return }

    let version = version_recognition(&input_bytes, &total_len, &mut acc_len, &mut index);
    let version = if version.is_none() { return } else { version.unwrap() };

    let istruction = version.parse(&input_bytes, &total_len, &mut acc_len, &mut index);
    let istruction = if istruction.is_none() { return } else { istruction.unwrap() };

    if !istruction.execute(socket, &input_bytes).await { return }
}


/// Parse the input bytes and check the protocol name according to nFTP protocol.
/// 
/// # Arguments
/// * `input_bytes` - the input bytes to parse.
/// * `total_len` - the total length of the input bytes array.
/// * `acc_len` - stands for "accumulator length". It serves as a temporary total length, for checks.
/// * `index` - the index from which to start parsing the input bytes array.
/// 
#[inline]
pub fn protocol_recognition(
    input_bytes: &Vec<u8>, 
    total_len: &usize, 
    acc_len: &mut usize, 
    index: &mut usize
) -> bool
{
    if (total_len <= acc_len) || 
        (from_utf8(&input_bytes[*index..*acc_len]).unwrap() != "nFTP") { 
        println!("The request doesn't start with `nFTP`");
        return false;
    }
    *index = *acc_len;
    *acc_len += 1;
    true
}


/// Parse the input bytes and check the version according to nFTP protocol.
/// 
/// # Arguments
/// * `input_bytes` - the input bytes to parse.
/// * `total_len` - the total length of the input bytes array.
/// * `acc_len` - stands for "accumulator length". It serves as a temporary total length, for checks.
/// * `index` - the index from which to start parsing the input bytes array.
/// 
#[inline]
pub fn version_recognition(
    input_bytes: &Vec<u8>, 
    total_len: &usize, 
    acc_len: &mut usize, 
    index: &mut usize
) -> Option<Box<dyn Version>>
{
    if total_len <= acc_len { 
        println!("The total length isn't enough for version recognition");
        return None;
    }
    let mask = 0b1111_0000u8;
    match (input_bytes[*index] & mask) >> 4 {
        // major
        1u8 => match input_bytes[*index] & (!mask) {
            // minor
            0u8 => return Some(Box::new(Version1_0)),
            _ => {
                println!("The minor version isn't correct");
                return None;
            }
        },  _ => {
            println!("The major version isn't correct");
            return None;
        }
    };
}


/// Parse the input bytes and check/return the request istruction according to nFTP protocol.
/// 
/// # Arguments
/// * `input_bytes` - the input bytes to parse.
/// * `total_len` - the total length of the input bytes array.
/// * `acc_len` - stands for "accumulator length". It serves as a temporary total length, for checks.
/// * `index` - the index from which to start parsing the input bytes array.
/// 
#[inline]
pub fn istruction_recognition(
    input_bytes: &Vec<u8>, 
    total_len: &usize, 
    acc_len: &mut usize, 
    index: &mut usize
) -> Option<u8>
{
    *index = *acc_len;
    *acc_len += 1;
    if total_len <= acc_len { 
        println!("The total length isn't enough for istruction recognition");
        return None;
    }
    Some(input_bytes[*index])
}


/// Parse the input bytes and check/return the paths according to nFTP protocol.
/// 
/// # Arguments
/// * `input_bytes` - the input bytes to parse.
/// * `total_len` - the total length of the input bytes array.
/// * `acc_len` - stands for "accumulator length". It serves as a temporary total length, for checks.
/// * `index` - the index from which to start parsing the input bytes array.
/// 
#[inline]
pub fn path_recognition(
    input_bytes: &Vec<u8>, 
    total_len: &usize, 
    acc_len: &mut usize, 
    index: &mut usize
) -> Option<Vec<PathBuf>>
{
    *index = *acc_len;
    *acc_len += 1;
    if total_len <= acc_len {
        println!("n_paths error");
        return None;
    }
    
    let n_paths = input_bytes[*index];
    if n_paths < 1 && n_paths > MAX_PATHS {
        println!("error, n_paths is {}", n_paths);
        return None;
    }

    let mut paths: Vec<PathBuf> = Vec::with_capacity(n_paths as usize);

    for _ in 0..n_paths {
        *index = *acc_len;
        *acc_len += 2;
        if total_len <= acc_len {
            println!("path dim error");
            return None;
        }

        let path_dimension: u16 = ((input_bytes[*index] as u16) << 8) | (input_bytes[*index + 1] as u16);
        *index = *acc_len;
        *acc_len += path_dimension as usize;

        if total_len < acc_len {
            println!("path error");
            return None;
        }

        let p = from_utf8(&input_bytes[*index..*acc_len]);
        if p.is_err() {
            println!("from_utf8 error");
            return None;
        }
        paths.push(PathBuf::from(p.unwrap()));
    }

    Some(paths)
}



#[cfg(test)]
pub mod test {

    /// Tests for the function "protocol_recognition"
    pub mod protocol_recognition_test {
        use super::super::*;

        #[test]
        fn protocol_recognition_should_pass() {
            let mut input_bytes: Vec<u8> = Vec::new();
            input_bytes.extend_from_slice(b"nFTP");
            input_bytes.push(0u8);
            let mut acc_len: usize = 4;
            let mut index: usize = 0;
            
            assert_eq!(protocol_recognition(&input_bytes, &input_bytes.len(), &mut acc_len, &mut index), true);
            assert_eq!(index, 4);
            assert_eq!(acc_len, 5);
        }

        #[test]
        fn protocol_recognition_with_total_len_equal_to_acc_len_should_return_err() {
            let mut input_bytes: Vec<u8> = Vec::new();
            input_bytes.extend_from_slice(b"nFTP");
            
            assert_eq!(protocol_recognition(&input_bytes, &input_bytes.len(), &mut 4, &mut 0), false);
        }

        #[test]
        fn protocol_recognition_with_nftp_lowercase_should_return_err() {
            let mut input_bytes: Vec<u8> = Vec::new();
            input_bytes.extend_from_slice(b"nftp");
            input_bytes.push(0u8);
            
            assert_eq!(protocol_recognition(&input_bytes, &input_bytes.len(), &mut 4, &mut 0), false);
        }

        #[test]
        fn protocol_recognition_with_error_in_nftp_name_should_return_err() {
            let mut input_bytes: Vec<u8> = Vec::new();
            input_bytes.extend_from_slice(b"nF");
            input_bytes.push(0u8);
            
            assert_eq!(protocol_recognition(&input_bytes, &input_bytes.len(), &mut 4, &mut 0), false);
        }   
    }

    /// Tests for the function "version_recognition"
    pub mod version_recognition_test {
        use super::super::*;

        #[test]
        fn version_recognition_test() {
            let version_1_0 = 0b0001_0000u8;
            let input_bytes: Vec<u8> = vec![version_1_0];
            let total_len = input_bytes.len();
            let mut acc_len = 0;
            let mut index = 0;

            let version = version_recognition(&input_bytes, &total_len, &mut acc_len, &mut index);
            assert!(version.is_some());
            assert_eq!(version.unwrap().get_version(), 0b0001_0000u8);
        }

        #[test]
        fn version_recognition_version_1_1_should_return_err() {
            let version_1_1 = 0b0001_0001u8;
            let input_bytes: Vec<u8> = vec![version_1_1];
            let total_len = input_bytes.len();
            let mut acc_len = 0;
            let mut index = 0;

            let version = version_recognition(&input_bytes, &total_len, &mut acc_len, &mut index);
            assert!(version.is_none());
        }

        #[test]
        fn version_recognition_version_2_0_should_return_err() {
            let version_2_0 = 0b0010_0000u8;
            let input_bytes: Vec<u8> = vec![version_2_0];
            let total_len = input_bytes.len();
            let mut acc_len = 0;
            let mut index = 0;

            let version = version_recognition(&input_bytes, &total_len, &mut acc_len, &mut index);
            assert!(version.is_none());
        }

    }
    
    pub mod path_recognition_test {
        use super::super::*;

        #[test]
        fn path_recognition_test() {
            let n_path: u8 = 2;
            let path_1_bytes = b"/dir_1/dir_2/dir_3/dir_4/long_long_file.mp4";
            let path_2_bytes = b"/dir_1/dir_2/dir_3/file.txt";

            let mut input_bytes: Vec<u8> = vec![];
            input_bytes.push(n_path);

            input_bytes.extend_from_slice(&43u16.to_be_bytes());
            input_bytes.extend_from_slice(path_1_bytes);

            input_bytes.extend_from_slice(&27u16.to_be_bytes());
            input_bytes.extend_from_slice(path_2_bytes);

            let total_len = input_bytes.len();
            let mut acc_len = 0;
            let mut index = 0;

            let paths = path_recognition(&input_bytes, &total_len, &mut acc_len, &mut index);
            assert!(paths.is_some());
        }
    }
}