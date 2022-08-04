
/// Response code OK
pub const RC_OK: u8 = 1;

/// Represents a response header for the nFTP protocol.
pub struct ResponseHeader {
    version_major: u8,
    version_minor: u8,
    response_code: u8,
    payload_dim: Option<u64>
}

impl ResponseHeader {
    pub fn new(version_major: u8, version_minor: u8, response_code: u8, payload_dim: Option<u64>) -> Self {
        ResponseHeader { version_major, version_minor, response_code, payload_dim }
    }

    /// Transforms the data of the struct into a vector of bytes.
    pub fn transform(&self) -> Vec<u8> {
        let mut output_bytes = Vec::with_capacity(
            if self.payload_dim.is_some() {14} else {6}
        );

        output_bytes.extend_from_slice(b"nFTP");
        output_bytes.push((self.version_major << 4) | self.version_minor);
        output_bytes.push(self.response_code);
        if self.payload_dim.is_some() {
            output_bytes.extend_from_slice(&self.payload_dim.unwrap().to_be_bytes());
        }

        output_bytes
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
    fn recompose_u64_from_bytes_should_return_the_correct_u64_number() {
        let number = 18_446_744_073_709_551_615u64;
        let bytes = number.to_be_bytes();
        assert_eq!(number, reassemble_u64_from_bytes(&bytes));
    }

    #[test]
    fn transform_should_return_the_correct_array_of_bytes() {
        let res = ResponseHeader::new(1, 2, 200, Some(18_446_744_073_709_551_615u64));
        let output_bytes = res.transform();

        assert_eq!(output_bytes.len(), 14);
        assert_eq!(&output_bytes[0..4], b"nFTP");
        assert_eq!((output_bytes[4] & 0b1111_0000) >> 4, 1u8);
        assert_eq!(output_bytes[4] & 0b0000_1111, 2u8);
        assert_eq!(output_bytes[5], 200u8);
        let payload_dim = reassemble_u64_from_bytes(&output_bytes[6..]);
        assert_eq!(payload_dim, 18_446_744_073_709_551_615u64);
    }

    #[test]
    fn transform_with_none_payload_dim_should_return_the_correct_array_of_bytes() {
        let res = ResponseHeader::new(1, 2, 200, None);
        let output_bytes = res.transform();

        assert_eq!(output_bytes.len(), 6);
        assert_eq!(&output_bytes[0..4], b"nFTP");
        assert_eq!((output_bytes[4] & 0b1111_0000) >> 4, 1u8);
        assert_eq!(output_bytes[4] & 0b0000_1111, 2u8);
        assert_eq!(output_bytes[5], 200u8);
    }
}