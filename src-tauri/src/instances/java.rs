use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};
use zip::ZipArchive;

use serde::{Deserialize, Serialize};

use crate::tools::{download::download, path::get_path, request::get};

use super::config::get_config;

#[derive(Debug, Serialize, Deserialize)]
struct Java {
    binary: Binary,
    release_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Binary {
    architecture: String,
    os: String,
    image_type: String,
    package: Package,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    checksum: String,
    link: String,
}

/// THIS FUNC ONLY FOR WINDOWS!!!
/// Downloading custom java for minecraft to make life easier for people who have windows
#[tauri::command(async)]
pub async fn intsall_java(instance_name: String, java_version: String) {
    // I send a request to get the java version for this OS
    let mut buff = String::new();
    std::io::Read::read_to_string(
        &mut get(&format!(
            "https://api.adoptium.net/v3/assets/latest/{java_version}/hotspot?architecture={0}&os={1}&image_type=jdk", std::env::consts::ARCH, std::env::consts::OS
        ))
        .await
        .unwrap(),
        &mut buff,
    )
    .unwrap();

    // I take the very first element. Because if you specify the data exactly, it returns only 1 - the last version
    let java: Vec<Java> = serde_json::from_str(&buff).unwrap();
    let path = &format!("java/{0}.zip", java[0].release_name);
    download(&java[0].binary.package.link, path, None).await;
    unzip(get_path(path), &java[0].release_name);

    // And save this to instance config
    let mut config = get_config(&instance_name);
    config.set_java_path(format!("java/{0}/bin/java.exe", &java[0].release_name));
    config.save_config();
}

fn unzip(path: PathBuf, name: &str) {
    let file = File::open(path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    let output_folder = &get_path(&format!("java/{name}"));
    if !output_folder.exists() {
        fs::create_dir(output_folder).unwrap();
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let output_path = output_folder.join(file.name());
        if file.is_dir() {
            fs::create_dir_all(&output_path).unwrap();
        } else {
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).unwrap();
                }
            }
            let mut output_file = File::create(output_path).unwrap();
            io::copy(&mut file, &mut output_file).unwrap();
        }
    }
}
