use std::path::Path;
use std::path::PathBuf;

/// Parse path in Json of version
///
/// sample:
/// ```
/// parse_path("ca/weblite/java-objc-bridge/1.1/java-objc-bridge-1.1.jar") -> "ca/weblite/java-objc-bridge/1.1"
/// ```
pub fn parse_path(path: &Path) -> PathBuf {
    let str_path = path.display().to_string();
    let components: Vec<&str> = str_path.split('/').collect();
    return Path::new(&(components[..components.len() - 1]).join("/")).to_path_buf();
}