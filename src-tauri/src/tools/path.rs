use std::path::{Path, PathBuf};

const PATH: &str = ".yogurt";

/// add to path ~/.yogurt/{path}
pub fn get_path(path: &str) -> PathBuf {
    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => panic!("Failed to get home directory"),
    };

    return Path::new(&home_dir).join(PATH).join(path);
}

/// Parse path in Json of version
///
/// # Example:
///
/// ```
/// use yogurt::tools::path::parse_path;
/// use std::path::{Path, PathBuf};
/// assert_eq!(
///     PathBuf::from("ca/weblite/java-objc-bridge/1.1"),
///     parse_path(Path::new(
///         "ca/weblite/java-objc-bridge/1.1/java-objc-bridge-1.1.jar"
///     ))
/// );
/// ```
pub fn parse_path(path: &Path) -> PathBuf {
    let str_path = path.display().to_string();
    let components: Vec<&str> = str_path.split('/').collect();
    return Path::new(&(components[..components.len() - 1]).join("/")).to_path_buf();
}
