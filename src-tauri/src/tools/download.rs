extern crate reqwest;
use std::fs;
use std::fs::File;
use std::io;

use futures::StreamExt;

use crate::tools::path;
use crate::tools::request;
use crate::tools::sha::verify_sha1sum;

pub struct DownloadFile {
    pub name: String,
    pub path: String,
    pub sha1: Option<String>,
}

/// This is a func for multi-threaded downloads of large count of files
pub async fn multithreading_download(files: Vec<DownloadFile>) {
    let mut task = Vec::new();
    for file in &files {
        task.push(async move { download(&file.name, &file.path, file.sha1.clone()).await });
    }
    let stream = futures::stream::iter(task).buffer_unordered(10);
    stream.collect::<Vec<_>>().await;
}

/// This func download file and save
/// Also checks if the file and its sha sum exist
/// You can leave an empty param sha1man to skip the check
pub async fn download(url: &str, file_path: &str, sha1sum: Option<String>) -> Result<(), String> {
    let path = path::get_path(file_path);

    if path.exists() && sha1sum != None {
        if verify_sha1sum(&path, &sha1sum.clone().unwrap_or_default())? {
            return Ok(());
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
        return Err(result.err());
    }

    let mut file = match File::create(&path) {
        Err(err) => return Err(err.to_string()),
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
                if sha1sum == None || verify_sha1sum(&path, &sha1sum.clone().unwrap_or_default())? {
                    break;
                }
                println!("Error sha1sum file: {}", &path.display().to_string())
            }
            Err(err) => {
                eprintln!("Failed to download file: {}", err);
            }
        }
    }

    Ok(())
}
