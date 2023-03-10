use std::fs::{File, self};
use std::io::Read;
use std::path::Path;

use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
    pub artifact: Download,
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
}

use crate::tools::download::download;
use crate::tools::path;

async fn fetch_dependency(url: &str, id: &str) -> Result<Package, Error> {
    let path = format!("version/{}/{}.json", id, id);
    download(url, path.as_str(), &"".to_string()).await;

    let mut file = File::open(path::get_path(path.as_str())).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let package: Package = serde_json::from_str(&buff).unwrap();
    Ok(package)
}

use crate::minecraft::{
    config::create_config,
    library::download_library,
    assets::download_assets
};

#[tauri::command(async)]
pub async fn get_minecraft(url: String, id: String, name: String, java_args: String) {
    match fetch_dependency(url.as_str(), id.as_str()).await {
        Ok(package) => {
            // Downloading the library for the selected version of minecraft
            download_library(package.libraries).await;
            // Downloading the assets for the selected version of minecraft
            download_assets(package.asset_index).await;
            // Downloading the client jar for the selected version of minecraft
            download(
                &package.downloads.client.url,
                format!("version/{}/{}.jar", &id, &id).as_str(),
                &package.downloads.client.sha1,
            )
            .await;
            create_config(
                name.as_str(),
                id.as_str(),
                format!("{}.jar", id).as_str(),
                "/usr/bin/java",
                java_args.as_str(),
            )
            .await;
            // Create instance folder
            let result = fs::create_dir_all(path::parse_path(Path::new(&format!("instance/{}", name))));
            if result.is_err() {
                panic!("Failed to create directory: {:?}", result.err());
            }
        }
        Err(error) => {
            println!("Error message: {}", error);
        }
    }
}
