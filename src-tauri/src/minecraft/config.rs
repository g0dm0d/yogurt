use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

use crate::tools::path::{self, get_path};

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub version: String,
    pub client: String,
    pub java_path: String,
    pub arguments: String,
}

/// generate default config
pub async fn create_config(
    name: &str,
    version: &str,
    client: &str,
    java_path: &str,
    arguments: &str,
) {
    let path = path::get_path("configs");
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(_) => {}
            Err(err) => {
                panic!("{}", err)
            }
        }
    }

    let mut file = fs::File::create(&path.join(format!("{}.toml", &name))).unwrap();

    let toml = toml::toml! {
        version = version
        client = client
        java_path = java_path
        arguments = arguments
    };
    write!(file, "{}", toml.to_string()).unwrap();
}

/// returns the names of all files in the configs folder (file name = instance name)
#[tauri::command]
pub fn get_all_instances() -> Vec<Instance> {
    let files = fs::read_dir(get_path("configs"))
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().unwrap_or_default() == "toml");

    let mut instances: Vec<Instance> = Vec::new();
    for file in files {
        instances.push(get_config(file.display().to_string().as_str()));
    }
    return instances;
}

/// return info about instance by name
pub fn get_config(name: &str) -> Instance {
    let path = get_path(format!("configs/{}.toml", name).as_str());
    let file = std::fs::read_to_string(path).unwrap();
    let data: Instance = toml::from_str(&file).expect("Error parsing TOML");
    return data;
}
