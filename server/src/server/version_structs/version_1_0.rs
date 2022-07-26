use crate::server::{version_trait::Version, parser::{istruction_recognition, path_recognition}};

pub struct Version1_0;

impl Version for Version1_0 {
    fn parse(&self,
        input_bytes: &Vec<u8>,
        total_len: &usize, 
        acc_len: &mut usize,
        index: &mut usize
    ) -> Result<(), String> 
    {
        let istruction = istruction_recognition(input_bytes, total_len, acc_len, index);
        if let Err(e) = istruction { 
            return Err(e);
        }

        let paths = path_recognition(input_bytes, total_len, acc_len, index);
        if let Err(e) = paths { 
            return Err(e); 
        }

        match istruction.unwrap() {
            0 => (), // LIST
            1 => (), // GET
            2 => (), // INSERT
            _ => return Err(String::from("Bad Istruction"))
        }

        Ok(())
    }

    fn get_version(&self) -> u8 {
        0b0001_0000u8
    }
}