use std::fs::File;
use std::io::Read;
use serde_json::from_str;
use tokio::process::Command as AsyncCommand;

use crate::minecraft::get_minecraft::Package;
use crate::tools::path::get_path;

use crate::minecraft::config::get_config;
use crate::minecraft::library::lib_os;

pub async fn run(username: &str, uuid: &str, token: &str, instance: &str) {
    // Open instance configuration 
    let config = get_config(instance);

    // Open version json file
    let mut file = File::open(get_path(&format!("version/{}/{}.json",config.version, config.version))).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let data: Package = from_str(&contents).unwrap();

    let mut libraries = Vec::new();
    for file in data.libraries {
        if !lib_os(&file) {
            println!("Library {} doesn't support", file.name);
            continue;
        }
        // create array for -cp arg
        let path = get_path(&file.downloads.artifact.path).display().to_string();
        libraries.push(path)
    }
    libraries.push(get_path(&format!("version/{}/{}.jar",config.version,config.client)).display().to_string());

    let mut minecraft = AsyncCommand::new(config.java_path);
    minecraft.arg("-Djava.library.path=".to_owned() + get_path(&format!("version/{}/natives", config.version)).to_str().unwrap())
        .arg("-Dminecraft.launcher.brand=yogurt")
        .arg("-Dminecraft.launcher.version=0.1")
        .arg("-cp")
        .arg(libraries.join(":"))
        .arg("--username")
        .arg(username)
        .arg(config.version)
        .arg("--assetsDir")
        .arg(get_path("assets"))
        .arg("--assetIndex")
        .arg(data.asset_index.id)
        .arg("--uuid")
        .arg("--accessToken")
        .arg(token)
        .arg("--userType")
        .arg("--microsoft")
        .arg("--versionType")
        .arg("--release");
}
