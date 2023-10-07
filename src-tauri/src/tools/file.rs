use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn read_file(path: &PathBuf) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}
