use serde_json::from_str;
use std::fs::File;
use std::io::Read;
use std::process::Command;

use crate::accounts::account::get_user;
use crate::minecraft::get_minecraft::Package;
use crate::tools::path::{fix_path, get_path};

use crate::instances::config::get_config;
use crate::minecraft::library::lib_os;

#[tauri::command(async)]
pub async fn run_minecraft(username: String, instance: String) {
    let mut user = get_user(&username);
    user.verify_minecraft_token().await;
    run(&username, &user.uuid, &user.minecraft_token, &instance);
}

pub fn run(username: &str, uuid: &str, token: &str, instance: &str) {
    // Open instance configuration
    let config = get_config(instance);

    // Open version json file
    let mut file = File::open(get_path(&format!(
        "versions/{}/{}.json",
        config.version, config.version
    )))
    .unwrap();
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
        let path: String = get_path(&fix_path(&format!(
            "libraries/{}",
            file.downloads.artifact.path
        )))
        .display()
        .to_string();
        libraries.push(path)
    }
    libraries.push(
        get_path(&fix_path(&format!(
            "versions/{}/{}",
            config.version, config.client
        )))
        .display()
        .to_string(),
    );

    let mut minecraft = Command::new(config.java_path);
    minecraft
        .arg(
            "-Djava.library.path=".to_owned()
                + get_path(&fix_path(&format!("versions/{}/natives", config.version)))
                    .to_str()
                    .unwrap(),
        )
        .arg("-Dminecraft.launcher.brand=yogurt")
        .arg("-Dminecraft.launcher.version=0.1")
        .arg("-cp")
        .arg(libraries.join(":"))
        .arg("net.minecraft.client.main.Main")
        .arg("--username")
        .arg(username)
        .arg("--version")
        .arg(config.version)
        .arg("--gameDir")
        .arg(get_path(&format!("instances/{instance}")))
        .arg("--assetsDir")
        .arg(get_path("assets"))
        .arg("--assetIndex")
        .arg(data.asset_index.id)
        .arg("--uuid")
        .arg(uuid)
        .arg("--accessToken")
        .arg(token)
        .arg("--userType")
        .arg("microsoft")
        .arg("--versionType")
        .arg("release");
    println!("{:?}", minecraft);
    let output = minecraft
        .output()
        .expect("Failed to start Minecraft client");
    println!("Minecraft client exited with status: {}", output.status);
    println!(
        "Minecraft client stdout: {:?}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!(
        "Minecraft client stderr: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
}
