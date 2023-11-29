use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Object {
    hash: String,
    size: i32,
}

impl Object {
    fn download_link(&self) -> String {
        let download_link = format!(
            "https://resources.download.minecraft.net/{0}/{1}",
            &self.hash[0..2],
            &self.hash
        );
        return download_link;
    }

    fn get_path(&self) -> String {
        let object_path = format!(
            "assets/objects/{0}/{1}",
            &self.hash[0..2],
            &self.hash
        );
        return object_path;
    }
}

#[derive(Serialize, Deserialize)]
struct Package {
    objects: HashMap<String, Object>,
}

use crate::tools::{download::{download, DownloadFile, multithreading_download}, path::get_path};

use super::get_minecraft::AssetIndex;

/// Parses json and download resource from json version of assetIndex[url] files are saved under their hash name, in a folder of 2 hash characters
///
/// Json version take from https://launchermeta.mojang.com/mc/game/version_manifest_v2.json
/// As sample https://piston-meta.mojang.com/v1/packages/d5274c45828abdd1bce21672f8e88f922536d391/1.19.3.json
/// And in 1.19.3.json assets link https://piston-meta.mojang.com/v1/packages/af25b63d7046b504c5b4fa7db05e639cad685978/2.json
pub async fn download_assets(assets_index: AssetIndex) -> Result<(), String>{
    let assets_path = format!("assets/indexes/{0}.json", assets_index.id);
    download(&assets_index.url, &assets_path, Some(assets_index.sha1)).await?;
    let file = std::fs::read_to_string(get_path(&assets_path))
        .expect("could not open the file with the index asstes");
    let assets: Package = serde_json::from_str(&file).expect("error json parsing");
    let mut task: Vec<DownloadFile> = Vec::new();
    for (_, asset) in &assets.objects {
        task.push(
            DownloadFile { 
                name: asset.download_link(),
                path: asset.get_path(),
                sha1: Some(asset.hash.clone())
            }
        )
    }
    multithreading_download(task).await;
    Ok(())
}
