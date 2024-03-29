use std::fs;
use std::time::Instant;

use crate::mods::fabric::install_fabric;
use crate::tools::download::download;
use crate::tools::file::read_file;
use crate::tools::path::get_path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Download {
    /// Path is always a relative parent folder ($HOME/.yogurt)
    pub path: String,
    /// sha1 sum for verify
    pub sha1: String,
    /// file size
    pub size: i32,
    /// download url
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<Download>,
    pub classifiers: Option<Classifiers>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classifiers {
    #[serde(rename = "natives-linux")]
    pub natives_linux: Option<Download>,
    #[serde(rename = "natives-windows")]
    pub natives_windows: Option<Download>,
    #[serde(rename = "natives-windows-32")]
    pub natives_windows_32: Option<Download>,
    #[serde(rename = "natives-windows-64")]
    pub natives_windows_64: Option<Download>,
    #[serde(rename = "natives-osx")]
    pub natives_osx: Option<Download>,
}

/// The structure that stores the name of the os for which this library is suitable
///
/// Struct:
/// LibraryOs
/// └── name: String
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryOs {
    /// name of os type
    pub name: String,
}

/// Library rule structure in the json version of minecraft
///
/// Struct:
/// LibraryRule
/// ├── action:     String (allow|???)
/// └── rules:      Vec<LibraryOs>
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryRule {
    pub action: String,
    pub os: Option<LibraryOs>,
}

/// Library structure in the json version of minecraft
///
/// Struct:
/// Library
/// ├── download:   LibraryDownloads
/// ├── name:       String
/// └── rules:      Vec<LibraryRule> | Optional
#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<LibraryRule>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JavaVersion {
    component: String,
    #[serde(rename = "majorVersion")]
    pub major_version: i32,
}

/// Info about json assets
///
/// Struct:
/// Library
/// ├── id:     String
/// ├── sha1:   String
/// └── url:    String
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetIndex {
    pub id: String,
    /// sha1 sum for verify
    pub sha1: String,
    /// download url
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    /// sha1 sum for verify
    sha1: String,
    /// download url
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Downloads {
    client: Client,
}

/// Json with version information
///
/// Struct:
/// Library
/// ├── asset_index:    AssetIndex
/// ├── downloads:      Downloads    
/// ├── id:             String    
/// ├── java_version:   String    
/// └── libraries:      Vec<Library>
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub downloads: Downloads,
    pub id: String,
    #[serde(rename = "javaVersion")]
    pub java_version: JavaVersion,
    pub libraries: Vec<Library>,
    #[serde(rename = "mainClass")]
    pub main_class: String,
}

async fn fetch_dependency(url: &str, id: &str) -> Result<Package, String> {
    let path = format!("versions/{id}/{id}.json");
    download(url, path.as_str(), None).await?;

    let file = read_file(&get_path(&path))?;

    let package: Package = serde_json::from_str(&file).map_err(|err| err.to_string())?;
    Ok(package)
}

use crate::minecraft::{assets::download_assets, library::download_libraries};

use crate::instances::config::{create_config, Instance};

#[tauri::command(async)]
pub async fn get_minecraft(
    url: String,
    id: String,
    name: String,
    java_args: String,
    fabric: bool,
) -> Result<(), String> {
    let package = fetch_dependency(url.as_str(), id.as_str())
        .await
        .map_err(|err| err.to_string())?;

    // Downloading the library for the selected version of minecraft
    let start = Instant::now();
    download_libraries(package.libraries).await;
    // Downloading the assets for the selected version of minecraft
    download_assets(package.asset_index).await?;
    let duration = start.elapsed();
    println!("Total time is: {:?}", duration);
    // Downloading the client jar for the selected version of minecraft
    download(
        &package.downloads.client.url,
        format!("versions/{id}/{id}.jar").as_str(),
        Some(package.downloads.client.sha1),
    )
    .await?;

    // generate config
    create_config(
        Instance {
            version: id.clone(),
            client: format!("{id}.jar"),
            java_path: "javaw".to_owned(),
            arguments: java_args,
            fabric,
            fabric_version: None,
            name: None,
        },
        name.as_str(),
    )
    .await?;

    // install fabric if need
    if fabric {
        install_fabric(name.clone()).await?;
    }
    // creating link for folder screenshots
    link_screenshots(&name);
    Ok(())
}

/// link screenshots folder of launcher and screenshots of instance
fn link_screenshots(name: &str) {
    let path = get_path(&format!("instances/{name}/screenshots"));
    let result = fs::create_dir_all(&path);
    if result.is_err() {
        panic!("Failed to create directory: {:?}", result.err());
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let result = std::os::unix::fs::symlink(path, get_path(&format!("screenshots/{name}")));
    #[cfg(target_os = "windows")]
    let result = std::os::windows::fs::symlink_dir(path, get_path(&format!("screenshots/{name}")));
    if result.is_err() {
        println!("{:?}", result.err());
    }
}
