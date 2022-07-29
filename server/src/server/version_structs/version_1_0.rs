use std::path::PathBuf;

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

    fn get_version(&self) -> u8 {
        0b0001_0000u8
    }
}


/// The GET istruction
pub struct Get {
    pub paths: Vec<PathBuf>
}
impl Istruction for Get {
    fn execute(&self) -> Result<(), String> {
        let main_path = PathBuf::from(MAIN_PATH);
        let mut new_vec = Vec::with_capacity(self.paths.len());

        for path in &self.paths {
            let complete_path = main_path.join(path);
            if !complete_path.exists() {
                return Err(String::from("A path doesn't exists"));
            }
            new_vec.push(complete_path);
        }

        let response_header = ResponseHeader::new(1, 0, RC_OK, Some(new_vec.len() as u16));

        Ok(())
    }

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