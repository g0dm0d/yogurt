use crate::tools::download::download;
use std::env;
use std::path::Path;

/// Downloading all the necessary libraries for the selected version of minecraft
///
/// Libs for version take from https://launchermeta.mojang.com/mc/game/version_manifest_v2.json
/// As sample https://piston-meta.mojang.com/v1/packages/d5274c45828abdd1bce21672f8e88f922536d391/1.19.3.json
pub async fn download_library(libraries: Vec<crate::minecraft::get_minecraft::Library>) {
    for file in libraries {
        if !lib_os(&file) {
            println!("Library {} doesn't support", file.name);
            continue;
        }
        download(
            &file.downloads.artifact.url,
            &Path::new("library").join(file.downloads.artifact.path),
            &file.downloads.artifact.sha1,
        )
        .await;
    }
}

/// This is a function that this library is suitable for the current OS
fn lib_os(file: &crate::minecraft::get_minecraft::Library) -> bool {
    if let Some(rules) = &file.rules {
        if let Some(rule) = rules.get(0) {
            if let Some(os) = &rule.os {
                if os.name != env::consts::OS {
                    return false;
                }
            }
        }
    }
    true
}
