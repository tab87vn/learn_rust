use core::panic;
pub use std::fs::File as File;

pub fn open_file(file_path: String) -> File {
    let f = File::open(file_path);
    let x = match f {
        Ok(file) => file,
        Err(error) => panic!("Error opening file {:?}", error)
    };

    x
    
}