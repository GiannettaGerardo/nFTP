/// Trait to represent all the versions of the nFTP protocol. 
pub trait Version {

    /// Continue the parsing of a specific version of the protocol.
    fn parse(&self,
        input_bytes: &Vec<u8>,
        total_len: &usize, 
        acc_len: &mut usize, 
        index: &mut usize) -> Result<Box<dyn Istruction>, String>;


    /// Return the version according to the nFTP protocol.
    /// 
    /// # Format
    /// * `0b0000_0000` - first 4 bit for the major, last 4 for bit for the minor of the protocol version.
    fn get_version(&self) -> u8;
    
}


/// Trait to represent all the istructions and their actions.
pub trait Istruction {
    fn execute(&self) -> Result<(), String>;

    fn get_istruction_code(&self) -> u8;
}