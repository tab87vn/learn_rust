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

pub fn open_file_two(path: String) -> File {
    let f = File::open(path);

    match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => match File::create(path)  {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_err => {
                panic!("Problem opening the file: {:?}", other_err)
            }
        }
    }
}