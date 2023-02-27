use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Object {
    hash: String,
    size: i32,
}

#[derive(Serialize, Deserialize)]
struct Package {
    objects: HashMap<String, Object>,
}

async fn fetch_assets(url: &str) -> Result<Package, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.json::<Package>().await?;
    Ok(response)
}

use crate::tools::download::download;

/// Parses json and download resource from json version of assetIndex[url] files are saved under their hash name, in a folder of 2 hash characters
///
/// Json version take from https://launchermeta.mojang.com/mc/game/version_manifest_v2.json
/// As sample https://piston-meta.mojang.com/v1/packages/d5274c45828abdd1bce21672f8e88f922536d391/1.19.3.json
/// And in 1.19.3.json assets link https://piston-meta.mojang.com/v1/packages/af25b63d7046b504c5b4fa7db05e639cad685978/2.json
pub async fn download_assets(assets_url: &str) {
    let assets = match fetch_assets(assets_url).await {
        Ok(assets) => assets,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    for (_, asset) in &assets.objects {
        println!("{}", asset.hash);
        download(
            &format!(
                "https://resources.download.minecraft.net/{}/{}",
                &asset.hash[0..2],
                &asset.hash
            ),
            &format!("assets/objects/{}/{}", &asset.hash[0..2], &asset.hash),
            &asset.hash,
        )
        .await
    }
}
