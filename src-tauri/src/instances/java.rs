use std::fs::{self, File};
use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::minecraft::get_minecraft;
use crate::tools::{download::download, path::get_path, request::get};

#[cfg(target_os = "windows")]
use crate::tools::archives::unzip;

#[cfg(target_os = "linux")]
use crate::tools::archives::untar;

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
const BINARY_FILE: &str = "javaw.exe";
#[cfg(target_os = "linux")]
const BINARY_FILE: &str = "javaw";

/// THIS FUNC ONLY FOR WINDOWS and now Linux
/// Downloading custom java for minecraft to make life easier for people who have windows
#[cfg(any(target_os = "linux", target_os = "windows"))]
#[tauri::command(async)]
pub async fn install_java(instance_name: String) {
    println!("starting download java");
    let mut config = get_config(&instance_name);

    // Get the java version for this instance
    let id = &config.version;
    let path = &format!("versions/{id}/{id}.json");

    let mut file = File::open(get_path(path)).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let package: get_minecraft::Package = serde_json::from_str(&buff).unwrap();
    let java_version = package.java_version.major_version.to_string();

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
    download(&java[0].binary.package.link, &format!("{path}.tar"), None).await;
    println!("java download complete");

    // This is necessary because linux releases are in tar.gz and windows releases are in .zip
    #[cfg(target_os = "windows")]
    unzip(get_path(&format!("{path}.tar")));
    #[cfg(target_os = "linux")]
    untar(get_path(&format!("{path}.tar")));
    println!("java installation complete");

    // And save this to instance config
    config.set_java_path(
        get_path(&format!(
            "java/{0}/bin/{1}",
            &java[0].release_name, BINARY_FILE
        ))
        .display()
        .to_string(),
    );
    config.save_config();

    // Deletes the archive. Since it is already garbage
    fs::remove_file(get_path(&format!("{path}.tar"))).unwrap();
}
