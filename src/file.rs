use std::{self, env, fs::File, io::Read, path::Path, process};

fn main() {
    // Get current file name and directory
    let filename = file!();
    println!("Current filename: {}", filename);
    println!("Current filename: {:?}", Path::new(&filename));

    let cwd = env::current_dir().unwrap();
    let open_file = File::open(cwd.join("README.md")); // The cwd starts from root project instead of this file.
    let mut file = match open_file {
        Ok(file) => {
            println!("{:?}", file);
            file
        }
        Err(err) => {
            println!("[FileError] {}", err);
            process::exit(0);
        }
    };
    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(byte_size) => {
            println!("Buffer size: {} bytes", byte_size);
            println!("File content {:?}", &buf);
        }
        Err(err) => {
            println!("Error reading file {}", err);
            process::exit(0);
        }
    };
}
