use std::fs;
use std::io::Write;
use std::path::Path;

use crate::tools::path;

pub async fn create_config(name: &str, version: &str, client: &str, java_path: &str, arguments: &str) {
    let path = path::get_path(Path::new("configs"));
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(_) => {}
            Err(err) => {
                panic!("{}", err)
            }
        }
    }

    let mut file = fs::File::create(path::get_path(&path.join(format!("{}.toml", &name)))).unwrap();

    let toml = toml::toml! {
        [instance]
        name = name
        version = version
        client = client
        java_path = java_path
        arguments = arguments
    };
    write!(file, "{}", toml.to_string()).unwrap();
}
