use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn is_binary(path: &Path) -> bool {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return true, // Assume binary if the file cannot be opened
    };

    let mut buffer = [0; 1024];
    let n = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(_) => return true,
    };

    buffer[..n].contains(&0)
}

pub fn format_size(size: u64) -> String {
    humansize::format_size(size, humansize::DECIMAL)
}
