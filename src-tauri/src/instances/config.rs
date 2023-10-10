use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use toml::to_string_pretty;

use crate::tools::path::{self, get_path};

/// Structure for real config
#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub version: String,
    pub client: String,
    pub java_path: String,
    pub arguments: String,
    pub fabric: bool,
    pub fabric_version: Option<String>,
    #[serde(skip)]
    pub name: Option<String>,
}

impl Instance {
    pub fn set_fabric_version(&mut self, version: String) {
        self.fabric_version = Some(version);
    }

    pub fn set_fabric_status(&mut self, status: bool) {
        self.fabric = status;
    }

    pub fn set_java_path(&mut self, path: String) {
        self.java_path = path
    }

    pub fn save_config(&mut self) {
        let path = get_path(&format!("configs/{0}.toml", self.name.clone().unwrap()));
        fs::write(path, toml::to_string(&self).unwrap()).unwrap()
    }
}

/// Structure for the front, to display the instance in the Launcher
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontInstance {
    pub name: String,
    pub version: String,
    pub game_type: String,
}

/// generate default config
pub async fn create_config(config: Instance, name: &str) -> Result<(), String> {
    let path = path::get_path("configs");
    if !path.exists() {
        std::fs::create_dir_all(&path).map_err(|err| err.to_string())?
    }

    let mut file =
        fs::File::create(&path.join(format!("{name}.toml"))).map_err(|err| err.to_string())?;
    let toml_string = to_string_pretty(&config).map_err(|err| err.to_string())?;
    file.write_all(toml_string.as_bytes())
        .map_err(|err| err.to_string())?;
    Ok(())
}

/// returns the names of all files in the configs folder (file name = instance name)
#[tauri::command]
pub fn get_all_instances() -> Vec<FrontInstance> {
    let files = fs::read_dir(get_path("configs"))
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().unwrap_or_default() == "toml");

    let mut instances: Vec<FrontInstance> = Vec::new();
    for file in files {
        if let Some(file_name) = file.file_stem() {
            if let Some(file_name_str) = file_name.to_str() {
                let config = get_config(file_name_str);
                instances.push(FrontInstance {
                    name: file_name_str.to_owned(),
                    version: config.version,
                    game_type: version_convector(config.fabric),
                });
            } else {
                println!("Could not convert file name to string");
            }
        } else {
            println!("Could not get file name");
        }
    }
    return instances;
}

fn version_convector(version: bool) -> String {
    if version {
        return "fabric".to_owned();
    }
    return "minecraft".to_owned();
}

/// return info about instance by name
pub fn get_config(name: &str) -> Instance {
    let path = get_path(format!("configs/{name}.toml").as_str());
    let file = std::fs::read_to_string(path).unwrap();
    let mut data: Instance = toml::from_str(&file).expect("Error parsing TOML");
    data.name = Some(name.to_owned());
    return data;
}
