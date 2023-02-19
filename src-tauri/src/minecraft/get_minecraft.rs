use std::fs::File;
use std::io::Read;
use std::path::Path;

use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Download {
    pub path: String,
    pub sha1: String,
    size: i32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Download,
}

/// The structure that stores the name of the os for which this library is suitable
///
/// Struct:
/// LibraryOs
/// └── name: String
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryOs {
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
    action: String,
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
struct JavaVersion {
    component: String,
    #[serde(rename = "majorVersion")]
    major_version: i32,
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
    id: String,
    sha1: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub sha1: String,
    pub url: String,
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
struct Package {
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndex,
    downloads: Downloads,
    id: String,
    #[serde(rename = "javaVersion")]
    java_version: JavaVersion,
    libraries: Vec<Library>,
}

async fn fetch_dependency(url: &str, id: &str) -> Result<Package, Error> {
    let str_path = format!("version/{}/{}.json", id, id);
    let path = Path::new(&str_path);
    download(url, path, &"".to_string()).await;

    let mut file = File::open(get_path(path)).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let package: Package = serde_json::from_str(&buff).unwrap();
    Ok(package)
}

use crate::minecraft::assets::download_assets;
use crate::minecraft::library::download_library;

use super::{
    config::create_config,
    download::{download, get_path},
};

#[tauri::command(async)]
pub async fn get_minecraft(url: String, id: String, name: String, java_args: String) {
    match fetch_dependency(url.as_str(), id.as_str()).await {
        Ok(package) => {
            // Downloading the library for the selected version of minecraft
            download_library(package.libraries).await;
            // Downloading the assets for the selected version of minecraft
            download_assets(package.asset_index.url.as_str()).await;
            // Downloading the client jar for the selected version of minecraft
            download(
                &package.downloads.client.url,
                Path::new(&format!("version/{}/{}.jar", &id, &id)),
                &package.downloads.client.sha1,
            )
            .await;
            create_config(
                name.as_str(),
                id.as_str(),
                "/usr/bin/java",
                java_args.as_str(),
            )
            .await;
        }
        Err(error) => {
            println!("Error message: {}", error);
        }
    }
}
