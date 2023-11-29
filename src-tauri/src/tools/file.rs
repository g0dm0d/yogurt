use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn read_file(path: &PathBuf) -> Result<String, String> {
    let mut file = File::open(path).map_err(|err| err.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| err.to_string())?;
    Ok(contents)
}

pub fn read_file_binary(path: &PathBuf) -> Result<Vec<u8>, String> {
    let mut file = File::open(path).map_err(|err| err.to_string())?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .map_err(|err| err.to_string())?;
    Ok(contents)
}
