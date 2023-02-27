use std::fs;
use std::io::Write;
use serde::Deserialize;

use crate::tools::path::{self, get_path};

#[derive(Deserialize)]
pub struct Instance {
    pub version: String,
    pub client: String,
    pub java_path: String,
    pub arguments: String,
}

pub async fn create_config(name: &str, version: &str, client: &str, java_path: &str, arguments: &str) {
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
        [instance]
        version = version
        client = client
        java_path = java_path
        arguments = arguments
    };
    write!(file, "{}", toml.to_string()).unwrap();
}

pub fn get_config(name: &str) -> Instance {
    let path = get_path(format!("configs/{}.toml", name).as_str());
    let file = std::fs::read_to_string(path).unwrap();
    let data: Instance = toml::from_str(&file).expect("Error parsing TOML");
    return data

}
