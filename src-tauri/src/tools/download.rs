extern crate reqwest;
use std::fs;
use std::fs::File;
use std::io;

use crate::tools::path;
use crate::tools::request;
use crate::tools::sha::verify_sha1sum;

/// This func download file and save
/// Also checks if the file and its sha sum exist
/// You can leave an empty param sha1man to skip the check
pub async fn download(url: &str, file_path: &str, sha1sum: Option<String>) {
    let path = path::get_path(file_path);

    if path.exists() && sha1sum != None {
        if verify_sha1sum(&path, &sha1sum.clone().unwrap_or_default()) {
            println!("File {} alredy exist", &path.display().to_string());
            return;
        }
        match fs::remove_file(&path) {
            Ok(_) => {
                println!("The file was broken {}", &path.display().to_string())
            }
            Err(e) => {
                println!("Error when delete broken file: {}", e)
            }
        };
    }

    let result = fs::create_dir_all(path::parse_path(&path));
    if result.is_err() {
        panic!("Failed to create directory: {:?}", result.err());
    }

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    loop {
        match request::get(url).await {
            Ok(mut content) => {
                io::copy(&mut content, &mut file).expect("Failed to read response body");
                let result = std::io::copy(&mut content, &mut file);

                if result.is_err() {
                    panic!("Failed to copy file: {:?}", result.err());
                }
                if sha1sum == None || verify_sha1sum(&path, &sha1sum.clone().unwrap_or_default()) {
                    break;
                }
                println!("Error sha1sum file: {}", &path.display().to_string())
            }
            Err(err) => {
                eprintln!("Failed to download file: {}", err);
            }
        }
    }
}
