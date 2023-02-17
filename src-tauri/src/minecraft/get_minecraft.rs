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
pub struct Downloads {
    client: Download,
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
    // downloads: Downloads,
    id: String,
    #[serde(rename = "javaVersion")]
    java_version: JavaVersion,
    libraries: Vec<Library>,
}

async fn fetch_dependency(url: String) -> Result<Package, Error> {
    let response: Package = reqwest::Client::new().get(url).send().await?.json().await?;
    Ok(response)
}

use crate::minecraft::assets::download_assets;
use crate::minecraft::library::download_library;

#[tauri::command(async)]
pub async fn get_minecraft(url: String) {
    match fetch_dependency(url).await {
        Ok(package) => {
            println!("{}", package.java_version.major_version);
            download_library(package.libraries).await;
            download_assets(package.asset_index.url).await;
        }
        Err(error) => {
            println!("Error message: {}", error);
        }
    }
}
