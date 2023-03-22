use crate::tools::download::{DownloadFile, multithreading_download};
use std::env;

/// Downloading all the necessary libraries for the selected version of minecraft
///
/// Libs for version take from https://launchermeta.mojang.com/mc/game/version_manifest_v2.json
/// As sample https://piston-meta.mojang.com/v1/packages/d5274c45828abdd1bce21672f8e88f922536d391/1.19.3.json
pub async fn download_libraries(libraries: Vec<crate::minecraft::get_minecraft::Library>) {
    let mut task: Vec<DownloadFile> = Vec::new();
    for file in libraries {
        if !lib_os(&file) {
            println!("Library {} doesn't support", file.name);
            continue;
        }
        task.push(
            DownloadFile { 
                name: file.downloads.artifact.url,
                path: format!("libraries/{0}", file.downloads.artifact.path),
                sha1: Some(file.downloads.artifact.sha1)
            }
        )
    }
    multithreading_download(task).await
}

/// This is a function that this library is suitable for the current OS
pub fn lib_os(file: &crate::minecraft::get_minecraft::Library) -> bool {
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
