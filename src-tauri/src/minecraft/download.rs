extern crate reqwest;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::tools::parse_path::parse_path;

const PATH: &str = ".yogurt";

pub async fn download(url: String, path: &Path) -> io::Result<()> {
    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => panic!("Failed to get home directory"),
    };
    let path = Path::new(&home_dir).join(PATH).join(path);
    fs::create_dir_all(parse_path(&path))?;
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let response = reqwest::get(url).await.expect("request failed");
    let mut content = io::Cursor::new(response.bytes().await.expect("body invalid"));
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
