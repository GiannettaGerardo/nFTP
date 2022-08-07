pub mod parser;
pub mod version_trait;
pub mod version_structs;
pub mod response;

use std::path::PathBuf;

use tokio::{net::TcpStream, io::AsyncReadExt};

// test
pub static MAIN_PATH: &str = "/home/gg/Scrivania/rust";

/// Read bytes from the socket and return the number of bytes readed.
#[inline]
async fn read_bytes(socket: &mut TcpStream, buf: &mut Vec<u8>) -> Option<usize> {
    match (*socket).read(&mut buf[..]).await {
        Ok(n) if n <= 0 => {
            println!("Read 0 bytes");
            return None;
        },
        Ok(n) => return Some(n),
        Err(e) => {
            println!("{}", e);
            return None;
        }
    }
}


/// Light Weight Deserialization for the tree structure of filesystem.
/// The rules are:
/// 1. for each directory, after the name use '{' and '}' to represent its contents;
/// 2. for each file, after the name use ',' to represent that it's a file;
/// 
/// # Examples
/// ```
/// /
/// |-> dir_1
/// |     |---> file.txt
/// |     |---> dir_2
/// |             |---> file.txt
/// |             |---> file.pdf
/// |-> dir_3
///       |---> dir_4
///               |---> dir_5
/// 
/// deserialization:
/// /{dir_1{file.txt,dir_2{file.txt,file.pdf}}dir_3{dir_4{dir_5{}}}}
/// ```
/// 
/// # Arguments
/// * `path` - the path from which to start the deserialization.
/// * `result` - the mutable string that will contain the result.
/// 
pub fn tree_deserialization(path: &PathBuf, result: &mut String) {
    let dir = std::fs::read_dir(&path).expect("It is impossibile to read this Directory...");

    result.push_str(path.file_name().unwrap().to_str().unwrap());
    result.push('{');

    for result_path in dir {
        let p = result_path.expect("It is impossible to get a path in the Directory...");
        
        if !p.file_type().unwrap().is_dir() {
            // is file
            result.push_str(p.file_name().to_str().unwrap());
            result.push(',');
            continue;
        }
        // is dir
        tree_deserialization(&path.join(p.file_name()), result);
    }

    result.push('}');
}


#[cfg(test)]
pub mod test {
    use std::path::PathBuf;

    use super::tree_deserialization;

    #[test]
    fn tree_deserialization_each_opening_parenthesis_is_properly_closed() {
        let mut result = String::new();
        let path = PathBuf::from("./tests/tree_deserialization/root");
        let mut counter = 0;

        tree_deserialization(&path, &mut result);

        for char in result.chars() {
            if char == '{' {
                counter += 1;
            }
            else if char == '}' {
                counter -= 1;
            }
            assert!(counter >= 0);
        }
        assert_eq!(counter, 0);
    }

    #[test]
    fn tree_deserialization_result_contains_all_the_strings_of_the_filesystem() {
        let mut result = String::new();
        let path = PathBuf::from("./tests/tree_deserialization/root");

        tree_deserialization(&path, &mut result);

        assert!(result.contains("root{"));
        assert!(result.contains("dir_1{"));
        assert!(result.contains("dir_2{dir_3{dir_4{}}}"));
        assert!(result.contains("dir_5{file_4.txt,}"));
        assert!(result.contains("file.txt,"));
        assert!(result.contains("file_1.txt,"));
        assert!(result.contains("file_3.txt,"));
    }
}