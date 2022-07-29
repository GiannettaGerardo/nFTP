use std::{str::from_utf8, path::PathBuf};
use super::{
    version_trait::Version, 
    version_structs::version_1_0::Version1_0
};

/// Parse the input bytes according to nFTP protocol.
pub fn process_request(input_bytes: Vec<u8>) -> Result<(), String> {
    let total_len = input_bytes.len();
    let mut acc_len: usize = 4;
    let mut index: usize = 0;
    
    if let Err(e) = protocol_recognition(&input_bytes, &total_len, &mut acc_len, &mut index) {
        return Err(e);
    }

    let version = version_recognition(&input_bytes, &total_len, &mut acc_len, &mut index);
    if let Err(e) = version {
        return Err(e);
    }
    else {
        let istruction = version.unwrap().parse(&input_bytes, &total_len, &mut acc_len, &mut index);
        if let Err(e) = istruction {
            return Err(e);
        }
        if let Err(e) = istruction.unwrap().execute() {
            return Err(e);
        }
    }
    Ok(())
}

/// Parse the input bytes and check the protocol name according to nFTP protocol.
/// 
/// # Arguments
/// * `input_bytes` - the input bytes to parse.
/// * `total_len` - the total length of the input bytes array.
/// * `acc_len` - stands for "accumulator length". It serves as a temporary total length, for checks.
/// * `index` - the index from which to start parsing the input bytes array.
/// 
pub fn protocol_recognition(
    input_bytes: &Vec<u8>, 
    total_len: &usize, 
    acc_len: &mut usize, 
    index: &mut usize
) -> Result<(), String>  
{
    if (total_len <= acc_len) || 
        (from_utf8(&input_bytes[*index..*acc_len]).unwrap() != "nFTP") { 
        return Err(String::from("The request doesn't start with `nFTP`")) 
    }
    *index = *acc_len;
    *acc_len += 1;
    Ok(())
}


/// Parse the input bytes and check the version according to nFTP protocol.
/// 
/// # Arguments
/// * `input_bytes` - the input bytes to parse.
/// * `total_len` - the total length of the input bytes array.
/// * `acc_len` - stands for "accumulator length". It serves as a temporary total length, for checks.
/// * `index` - the index from which to start parsing the input bytes array.
/// 
pub fn version_recognition(
    input_bytes: &Vec<u8>, 
    total_len: &usize, 
    acc_len: &mut usize, 
    index: &mut usize
) -> Result<Box<dyn Version>, String> 
{
    if total_len <= acc_len { 
        return Err(String::from("The total length isn't enough for version recognition")) 
    }
    let mask = 0b1111_0000u8;
    match (input_bytes[*index] & mask) >> 4 {
        // major
        1u8 => match input_bytes[*index] & (!mask) {
            // minor
            0u8 => return Ok(Box::new(Version1_0)),
            _ => return Err(String::from("The minor version isn't correct"))
        },
        _ => return Err(String::from("The major version isn't correct"))
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
pub fn istruction_recognition(
    input_bytes: &Vec<u8>, 
    total_len: &usize, 
    acc_len: &mut usize, 
    index: &mut usize
) -> Result<u8, String>
{
    *index = *acc_len;
    *acc_len += 1;
    if total_len <= acc_len { 
        return Err(String::from("The total length isn't enough for istruction recognition")) 
    }
    Ok(input_bytes[*index])
}


/// Parse the input bytes and check/return the paths according to nFTP protocol.
/// 
/// # Arguments
/// * `input_bytes` - the input bytes to parse.
/// * `total_len` - the total length of the input bytes array.
/// * `acc_len` - stands for "accumulator length". It serves as a temporary total length, for checks.
/// * `index` - the index from which to start parsing the input bytes array.
/// 
pub fn path_recognition(
    input_bytes: &Vec<u8>, 
    total_len: &usize, 
    acc_len: &mut usize, 
    index: &mut usize
) -> Result<Vec<PathBuf>, String>
{
    *index = *acc_len;
    *acc_len += 1;
    if total_len <= acc_len { return Err(String::from("n_paths error")) }
    
    let n_paths = input_bytes[*index];
    let mut paths: Vec<PathBuf> = Vec::with_capacity(n_paths as usize);

    for _ in 0..n_paths {
        *index = *acc_len;
        *acc_len += 2;
        if total_len <= acc_len { return Err(String::from("path_dim error")) }

        let path_dimension: u16 = ((input_bytes[*index] as u16) << 8) | (input_bytes[*index + 1] as u16);
        *index = *acc_len;
        *acc_len += path_dimension as usize;
        if total_len < acc_len { return Err(String::from("path error")) }

        let p = from_utf8(&input_bytes[*index..*acc_len]);
        if p.is_err() { return Err(String::from("from_utf8 error")) }
        paths.push(PathBuf::from(p.unwrap()));
    }

    Ok(paths)
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
            
            assert_eq!(protocol_recognition(&input_bytes, &input_bytes.len(), &mut acc_len, &mut index), Ok(()));
            assert_eq!(index, 4);
            assert_eq!(acc_len, 5);
        }

        #[test]
        fn protocol_recognition_with_total_len_equal_to_acc_len_should_return_err() {
            let mut input_bytes: Vec<u8> = Vec::new();
            input_bytes.extend_from_slice(b"nFTP");
            
            assert!(protocol_recognition(&input_bytes, &input_bytes.len(), &mut 4, &mut 0).is_err());
        }

        #[test]
        fn protocol_recognition_with_nftp_lowercase_should_return_err() {
            let mut input_bytes: Vec<u8> = Vec::new();
            input_bytes.extend_from_slice(b"nftp");
            input_bytes.push(0u8);
            
            assert!(protocol_recognition(&input_bytes, &input_bytes.len(), &mut 4, &mut 0).is_err());
        }

        #[test]
        fn protocol_recognition_with_error_in_nftp_name_should_return_err() {
            let mut input_bytes: Vec<u8> = Vec::new();
            input_bytes.extend_from_slice(b"nF");
            input_bytes.push(0u8);
            
            assert!(protocol_recognition(&input_bytes, &input_bytes.len(), &mut 4, &mut 0).is_err());
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
            assert!(version.is_ok());
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
            assert!(version.is_err());
        }

        #[test]
        fn version_recognition_version_2_0_should_return_err() {
            let version_2_0 = 0b0010_0000u8;
            let input_bytes: Vec<u8> = vec![version_2_0];
            let total_len = input_bytes.len();
            let mut acc_len = 0;
            let mut index = 0;

            let version = version_recognition(&input_bytes, &total_len, &mut acc_len, &mut index);
            assert!(version.is_err());
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
            assert!(paths.is_ok());
        }
    }
}