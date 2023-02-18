use std::fs;
use std::io::Write;
use std::path::Path;

use super::download::get_path;

pub async fn create_config(name: String, version: String, java_path: String, arguments: String) {
    let path = get_path(Path::new("configs"));
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(_) => {}
            Err(err) => {
                panic!("{}", err)
            }
        }
    }

    let mut file = fs::File::create(get_path(&path.join(format!("{}.toml", &name)))).unwrap();

    let toml = toml::toml! {
        [instance]
        name = name
        version = version
        java_path = java_path
        arguments = arguments
    };
    write!(file, "{}", toml.to_string()).unwrap();
}
