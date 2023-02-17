use std::path::Path;
use std::path::PathBuf;

pub fn parse_path(path: &Path) -> PathBuf {
    let str_path = path.display().to_string();
    let components: Vec<&str> = str_path.split('/').collect();
    return Path::new(&(components[..components.len() - 1]).join("/")).to_path_buf();
}
