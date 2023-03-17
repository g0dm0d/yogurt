use std::fs;

use serde::{Deserialize, Serialize};

use crate::tools::{
    archives::{untar, unzip},
    download::download,
    path::get_path,
    request::get,
};

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

#[cfg(target_os = "windows")]
const BINARY_FILE: &str = "java.exe";
#[cfg(target_os = "linux")]
const BINARY_FILE: &str = "java";

/// THIS FUNC ONLY FOR WINDOWS and now Linux
/// Downloading custom java for minecraft to make life easier for people who have windows
#[tauri::command(async)]
pub async fn install_java(instance_name: String, java_version: String) {
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
    let path = &format!("java/{0}", java[0].release_name);
    download(&java[0].binary.package.link, path, None).await;

    // This is necessary because linux releases are in tar.gz and windows releases are in .zip
    #[cfg(target_os = "windows")]
    unzip(get_path(path));
    #[cfg(target_os = "linux")]
    untar(get_path(path));

    // And save this to instance config
    let mut config = get_config(&instance_name);
    config.set_java_path(format!(
        "java/{0}/bin/{1}",
        &java[0].release_name, BINARY_FILE
    ));
    config.save_config();

    // Deletes the archive. Since it is already garbage
    fs::remove_file(path).unwrap();
}
