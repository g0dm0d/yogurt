use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

use crate::{
    instances::config::get_config,
    tools::{download::download, path::get_path, request::get},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct FabricData {
    pub libraries: Vec<Library>,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    pub arguments: Arguments,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Arguments {
    pub jvm: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Library {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Libraries {
    pub libraries: Vec<Library>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Loader {
    version: String,
}

#[tauri::command(async)]
pub async fn install_fabric(name: String) {
    let mut config = get_config(&name);
    // get last version loader from - https://meta.fabricmc.net/v2/versions/loader
    let loader = get_last_loader().await.unwrap();

    let fabric_version = format!("fabric-{0}-{1}", loader, config.version);

    // Downloading json for fabric
    // as sample https://meta.fabricmc.net/v2/versions/loader/1.19.3/0.14.17/profile/json
    download(
        &format!(
            "https://meta.fabricmc.net/v2/versions/loader/{0}/{1}/profile/json",
            config.version, loader
        ),
        &get_path(&format!("versions/{fabric_version}/{fabric_version}.json"))
            .display()
            .to_string(),
        None,
    )
    .await;

    let libraries = parse_libraries(&fabric_version);
    download_libraries(libraries).await;

    config.set_fabric_version(fabric_version);
    config.set_fabric_status(true);

    let mut file = File::create(get_path(&format!("configs/{name}.toml"))).unwrap();
    let toml_string = toml::to_string_pretty(&config).unwrap();
    file.write_all(toml_string.as_bytes()).unwrap();
}

async fn get_last_loader() -> Result<String, reqwest::Error> {
    let response: Vec<Loader> = reqwest::Client::new()
        .get("https://meta.fabricmc.net/v2/versions/loader")
        .send()
        .await?
        .json()
        .await?;
    Ok(response[0].version.clone())
}

/// This function parse the name in the path
///
/// # Example
/// ```
/// let library = parse_library("net.fabricmc:tiny-mappings-parser:0.3.0+build.17");
/// assert_eq!(library, "net/fabricmc/tiny-mappings-parser/0.3.0+build.17/tiny-mappings-parser-0.3.0+build.17.jar");
/// ```
fn parse_library(name: String) -> String {
    // I have name net.fabricmc:tiny-mappings-parser:0.3.0+build.17
    // this name eq net/fabricmc/tiny-mappings-parser/0.3.0+build.17/tiny-mappings-parser-0.3.0+build.17.jar
    // here I split this string to
    // Vec[0] = net.fabricmc
    // Vec[1] = tiny-mappings-parser
    // Vec[2] = 0.3.0+build.17
    let library = name.split(":").collect::<Vec<&str>>();
    // Here I do concatenation to get the file name -> tiny-mappings-parser-0.3.0+build.17.jar
    let filename = format!("{0}-{1}.jar", library[1], library[2]);
    // getting net/fabricmc/tiny-mappings-parser/0.3.0+build.17/tiny-mappings-parser-0.3.0+build.17.jar
    return format!(
        "{0}/{1}/{2}/{3}",
        // Replacing it to get net/fabricmc
        library[0].replace(".", "/"),
        library[1],
        library[2],
        filename
    );
}

/// This function parses all libraries in the json version
/// This is need to run minecraft
pub fn parse_libraries(version: &str) -> Vec<String> {
    let file = std::fs::read_to_string(get_path(&format!("versions/{}/{}.json", version, version)))
        .expect("could not open the file with the index asstes");
    let libraries: Libraries = serde_json::from_str(&file).expect("error json parsing");
    let mut libraries_str: Vec<String> = Vec::new();
    for library in libraries.libraries {
        libraries_str.push(
            get_path(&format!("libraries/{}", parse_library(library.name)))
                .display()
                .to_string(),
        )
    }
    return libraries_str;
}

async fn download_libraries(libraries: Vec<String>) {
    for library in libraries {
        let mut response = get(&format!("https://maven.fabricmc.net/{library}.sha1"))
            .await
            .unwrap();
        let mut buf = Vec::new();
        response.read_to_end(&mut buf).await.unwrap();
        let sha1 = String::from_utf8(buf).unwrap();
        println!("{}", sha1);
        download(
            &format!("https://maven.fabricmc.net/{library}"),
            &library,
            Some(sha1),
        )
        .await
    }
}
