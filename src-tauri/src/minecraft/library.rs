use crate::tools::download::{multithreading_download, DownloadFile};
use std::env;

use super::get_minecraft::{Download, Library};

/// Downloading all the necessary libraries for the selected version of minecraft
///
/// Libs for version take from https://launchermeta.mojang.com/mc/game/version_manifest_v2.json
/// As sample https://piston-meta.mojang.com/v1/packages/d5274c45828abdd1bce21672f8e88f922536d391/1.19.3.json
pub async fn download_libraries(libraries: Vec<Library>) {
    let mut task: Vec<DownloadFile> = Vec::new();
    for lib in library_filtering(&libraries) {
        task.push(DownloadFile {
            name: lib.url,
            path: format!("libraries/{0}", lib.path),
            sha1: Some(lib.sha1),
        });
    }
    multithreading_download(task).await
}

pub fn library_filtering(libs: &Vec<Library>) -> Vec<Download> {
    let mut filtred_libs: Vec<Download> = Vec::new();
    for lib in libs {
        if !lib_os(&lib) {
            println!("Library {} doesn't support", lib.name);
            continue;
        }

        match find_classifiers_lib(&lib) {
            Some(ref library) => filtred_libs.push(library.clone()),
            None => {}
        }

        match find_artifact_lib(&lib) {
            Some(ref library) => filtred_libs.push(library.clone()),
            None => {}
        }
    }
    return filtred_libs;
}

pub fn find_artifact_lib(library: &Library) -> Option<Download> {
    if let Some(lib) = &library.downloads.artifact {
        return Some(lib.clone());
    }
    return None;
}

/// this function is to check if there is a library for the given os. And if there is, then return it
pub fn find_classifiers_lib(library: &Library) -> Option<Download> {
    if let Some(lib) = &library.downloads.classifiers {
        #[cfg(target_os = "windows")]
        if let Some(l) = &lib.natives_windows_32 {
            if std::env::consts::ARCH == "x86" {
                return Some(l.clone());
            }
        }

        #[cfg(target_os = "windows")]
        if let Some(l) = &lib.natives_windows_64 {
            if std::env::consts::ARCH == "x86_64" {
                return Some(l.clone());
            }
        }

        #[cfg(target_os = "windows")]
        if let Some(l) = &lib.natives_windows {
            return Some(l.clone());
        }

        #[cfg(target_os = "macos")]
        if let Some(l) = &lib.natives_osx {
            return Some(l.clone());
        }

        #[cfg(target_os = "linux")]
        if let Some(l) = &lib.natives_linux {
            return Some(l.clone());
        }
        return None;
    }
    return None;
}

/// This is a function that this library is suitable for the current OS
pub fn lib_os(file: &Library) -> bool {
    if let Some(rules) = &file.rules {
        for rule in rules {
            if let Some(os) = &rule.os {
                if os.name == env::consts::OS && rule.action == "disallow" {
                    return false;
                }
            }
        }
    }
    true
}
