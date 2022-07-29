
/// Response code OK
pub const RC_OK: u8 = 1;

pub struct ResponseHeader {
    version_major: u8,
    version_minor: u8,
    response_code: u8,
    n_paths: Option<u16>
}

impl ResponseHeader {
    pub fn new(
        version_major: u8, version_minor: u8, 
        response_code: u8, n_paths: Option<u16>
    ) -> Self {
        ResponseHeader { 
            version_major, version_minor, 
            response_code, n_paths 
        }
    }

    pub fn transform(&self) -> Vec<u8> {
        let mut output_bytes = Vec::with_capacity(
            if self.n_paths.is_some() { 8 } else { 6 }
        );

        output_bytes.extend_from_slice(b"nFTP");
        output_bytes.push((self.version_major << 4) | self.version_minor);
        output_bytes.push(self.response_code);
        if self.n_paths.is_some() {
            output_bytes.extend_from_slice(&self.n_paths.unwrap().to_be_bytes());
        }

        output_bytes
    }
}


#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn transform_should_return_the_correct_array_of_bytes() {
        let res = ResponseHeader::new(1, 2, 200, Some(2));
        let output_bytes = res.transform();

        assert_eq!(output_bytes.len(), 8);
        assert_eq!(&output_bytes[0..4], b"nFTP");
        assert_eq!((output_bytes[4] & 0b1111_0000) >> 4, 1u8);
        assert_eq!(output_bytes[4] & 0b0000_1111, 2u8);
        assert_eq!(output_bytes[5], 200u8);
        assert_eq!(((output_bytes[6] as u16) << 8) | output_bytes[7] as u16, 2u16);
    }

    #[test]
    fn transform_with_none_n_paths_should_return_correct_array_of_bytes() {
        let res = ResponseHeader::new(1, 2, 200, None);
        let output_bytes = res.transform();

        assert_eq!(output_bytes.len(), 6);
        assert_eq!(&output_bytes[0..4], b"nFTP");
        assert_eq!((output_bytes[4] & 0b1111_0000) >> 4, 1u8);
        assert_eq!(output_bytes[4] & 0b0000_1111, 2u8);
        assert_eq!(output_bytes[5], 200u8);
    }
}