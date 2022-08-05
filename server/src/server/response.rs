/// Response code OK
pub const RC_OK: u8 = 1;

/// Represents a response header for the nFTP protocol.
pub struct ResponseHeader {
    header_bytes: Vec<u8>
}
impl ResponseHeader {
    pub fn new(version_major: u8, version_minor: u8, response_code: u8, payload_dim: Option<u64>) -> Self {
        let mut output_bytes = Vec::with_capacity(
            if payload_dim.is_some() {14} else {6}
        );

        output_bytes.extend_from_slice(b"nFTP");
        output_bytes.push((version_major << 4) | version_minor);
        output_bytes.push(response_code);
        if payload_dim.is_some() {
            output_bytes.extend_from_slice(&payload_dim.unwrap().to_be_bytes());
        }

        ResponseHeader { header_bytes: output_bytes }
    }

    #[inline]
    pub fn set_new_version(&mut self, version_major: u8, version_minor: u8) {
        self.header_bytes[4] = (version_major << 4) | version_minor;
    }

    #[inline]
    pub fn set_new_response_code(&mut self, response_code: u8) {
        self.header_bytes[5] = response_code;
    }

    #[inline]
    pub fn set_new_payload_dim(&mut self, payload_dim: Option<u64>) {
        if self.header_bytes.len() > 6 {
            match payload_dim {
                Some(payload_dim) => {
                    let bytes = payload_dim.to_be_bytes();
                    for i in 6..14 {
                        self.header_bytes[i] = bytes[i-6];
                    }
                },
                None => for _ in 0..8 { 
                    self.header_bytes.pop(); 
                }
            };
        } else if payload_dim.is_some() {
            let bytes = payload_dim.unwrap().to_be_bytes();
            for i in 0..8 {
                self.header_bytes.push(bytes[i]);
            }
        }
    }

    #[inline]
    pub fn get_header(&self) -> &Vec<u8> {
        &self.header_bytes
    }
}


/// Reassembles a vector of 8 bytes into a 64-bit unsigned integer.
/// Utility function.
pub fn reassemble_u64_from_bytes(bytes: &[u8]) -> u64 {
    let mut res: u64 = 0;
    let mut b: u8 = 64;
    for i in 0..8 {
        b -= 8;
        res |= (bytes[i] as u64) << b;
    }
    res
}


#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn reassemble_u64_from_bytes_should_return_the_correct_u64_number() {
        let number = 18_446_744_073_709_551_615u64;
        let bytes = number.to_be_bytes();
        assert_eq!(number, reassemble_u64_from_bytes(&bytes));
    }

    #[test]
    fn response_header_should_return_the_correct_array_of_bytes() {
        let res = ResponseHeader::new(1, 2, 200, Some(18_446_744_073_709_551_615u64));
        let output_bytes = res.get_header();

        assert_eq!(output_bytes.len(), 14);
        assert_eq!(&output_bytes[0..4], b"nFTP");
        assert_eq!((output_bytes[4] & 0b1111_0000) >> 4, 1u8);
        assert_eq!(output_bytes[4] & 0b0000_1111, 2u8);
        assert_eq!(output_bytes[5], 200u8);
        let payload_dim = reassemble_u64_from_bytes(&output_bytes[6..]);
        assert_eq!(payload_dim, 18_446_744_073_709_551_615u64);
    }

    #[test]
    fn response_header_with_none_payload_dim_should_return_the_correct_array_of_bytes() {
        let res = ResponseHeader::new(1, 2, 200, None);
        let output_bytes = res.get_header();

        assert_eq!(output_bytes.len(), 6);
        assert_eq!(&output_bytes[0..4], b"nFTP");
        assert_eq!((output_bytes[4] & 0b1111_0000) >> 4, 1u8);
        assert_eq!(output_bytes[4] & 0b0000_1111, 2u8);
        assert_eq!(output_bytes[5], 200u8);
    }

    #[test]
    fn response_header_after_setting_new_payload_dim_should_contains_the_new_payload_dim() {
        let mut h = ResponseHeader::new(1, 0, 200, Some(18_446_744_073_709_551_615u64));
        
        let new_payload_dimension = 17_000_744_111_709_555_001u64;
        h.set_new_payload_dim(Some(new_payload_dimension));
        assert_eq!(reassemble_u64_from_bytes(&h.get_header()[6..]), new_payload_dimension);
    }

    #[test]
    fn response_header_after_delete_payload_dim_should_not_contains_payload_dim() {
        let mut h = ResponseHeader::new(1, 0, 200, Some(18_446_744_073_709_551_615u64));
        
        h.set_new_payload_dim(None);
        assert_eq!(h.get_header().len(), 6);
    }

    #[test]
    fn response_header_without_payload_dim_should_have_len_6_after_setting_none() {
        let mut h = ResponseHeader::new(1, 0, 200, None);
        
        h.set_new_payload_dim(None);
        assert_eq!(h.get_header().len(), 6);
    }

    #[test]
    fn response_header_without_payload() {
        let mut h = ResponseHeader::new(1, 0, 200, None);
        
        let new_payload_dimension = 17_000_744_111_709_555_001u64;
        h.set_new_payload_dim(Some(new_payload_dimension));
        assert_eq!(reassemble_u64_from_bytes(&h.get_header()[6..]), new_payload_dimension);
    }
}