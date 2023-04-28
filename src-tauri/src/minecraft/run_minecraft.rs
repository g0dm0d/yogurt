use serde_json::from_str;
use std::process::Command;

use crate::accounts::account::get_user;
use crate::minecraft::get_minecraft::Package;
use crate::mods::fabric::{parse_libraries, FabricData};
use crate::tools::file::read_file;
use crate::tools::path::get_path;

use crate::instances::config::get_config;
use crate::minecraft::library::library_filtering;

#[tauri::command(async)]
pub async fn run_minecraft(username: String, instance: String) {
    let mut user = get_user(&username);
    user.verify_minecraft_token().await;
    run(&username, &user.uuid, &user.minecraft_token, &instance);
}

#[cfg(target_os = "windows")]
const SEP: &str = ";";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const SEP: &str = ":";

pub fn run(username: &str, uuid: &str, token: &str, instance: &str) {
    // Open instance configuration
    let config = get_config(instance);

    // Open version json file
    let minecraft_config = read_file(&format!(
        "versions/{}/{}.json",
        config.version, config.version
    ));
    let data: Package = from_str(&minecraft_config).unwrap();
    let mut user_args: Vec<String> = config
        .arguments
        .split_whitespace()
        .map(String::from)
        .collect();
    let mut main_class = data.main_class;

    let mut libraries = Vec::new();
    for lib in library_filtering(&data.libraries) {
        // create array for -cp arg
        let path: String = get_path(&format!("libraries/{}", lib.path))
            .display()
            .to_string();
        libraries.push(path)
    }

    if config.fabric {
        let fabric_version = config.fabric_version.unwrap();
        println!("{}", fabric_version);
        let libs = parse_libraries(&fabric_version);
        for lib in libs {
            libraries.push(
                get_path(&format!("libraries/{}", lib))
                    .display()
                    .to_string(),
            )
        }
        let fabric_config = read_file(&format!(
            "versions/{}/{}.json",
            fabric_version, fabric_version
        ));
        let fabric_data: FabricData = from_str(&fabric_config).unwrap();
        let jvm_args = fabric_data.arguments.jvm;
        for arg in jvm_args {
            user_args.push(arg)
        }
        main_class = fabric_data.main_class
    }
    libraries.push(
        get_path(&format!("versions/{}/{}", config.version, config.client))
            .display()
            .to_string(),
    );

    let mut minecraft = Command::new(config.java_path);
    minecraft
        .arg(
            "-Djava.library.path=".to_owned()
                + &format!("versions/{}/native/linux", config.version),
        )
        .arg("-Dminecraft.launcher.brand=yogurt")
        .arg("-Dminecraft.launcher.version=0.1")
        .args(user_args)
        .arg("-cp")
        .arg(libraries.join(SEP))
        .arg(main_class)
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
