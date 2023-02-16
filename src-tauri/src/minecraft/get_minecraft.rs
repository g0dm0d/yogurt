use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Download {
    sha1: String,
    size: i32,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LibraryDownloads {
    artifact: Download,
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
struct Library {
    downloads: LibraryDownloads,
    name: String,
    rules: Option<Vec<LibraryRule>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JavaVersion {
    component: String,
    majorVersion: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    downloads: serde_json::Value,
    id: String,
    javaVersion: JavaVersion,
    libraries: Vec<Library>,
}

async fn fetch_dependency(url: String) -> Result<Package, Error> {
    let response: Package = reqwest::Client::new().get(url).send().await?.json().await?;
    Ok(response)
}

#[tauri::command(async)]
pub async fn get_minecraft(url: String) -> String {
    println!("{}", url);
    let test = "xdd".to_string();
    match fetch_dependency(url).await {
        Ok(package) => {
            println!("Dependency name: {}", package.javaVersion.majorVersion);
        }
        Err(error) => {
            println!("Error message: {}", error);
        }
    }
    return test;
}
