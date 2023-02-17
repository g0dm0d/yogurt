use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Download {
    pub path: String,
    sha1: String,
    size: i32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Download,
}

#[derive(Debug, Serialize, Deserialize)]
struct LibraryOs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LibraryRule {
    action: String,
    os: Option<LibraryOs>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub downloads: LibraryDownloads,
    name: String,
    rules: Option<Vec<LibraryRule>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JavaVersion {
    component: String,
    #[serde(rename = "majorVersion")]
    major_version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    downloads: serde_json::Value,
    id: String,
    #[serde(rename = "javaVersion")]
    java_version: JavaVersion,
    libraries: Vec<Library>,
}

async fn fetch_dependency(url: String) -> Result<Package, Error> {
    let response: Package = reqwest::Client::new().get(url).send().await?.json().await?;
    Ok(response)
}

use crate::minecraft::library::download_library;

#[tauri::command(async)]
pub async fn get_minecraft(url: String) {
    match fetch_dependency(url).await {
        Ok(package) => {
            println!("{}", package.java_version.major_version);
            download_library(package.libraries).await;
        }
        Err(error) => {
            println!("Error message: {}", error);
        }
    }
}
