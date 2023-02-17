use crate::minecraft::download::download;
use std::path::Path;

pub async fn download_library(libraries: Vec<crate::minecraft::get_minecraft::Library>) {
    for file in libraries {
        download(
            file.downloads.artifact.url,
            &Path::new("Library").join(file.downloads.artifact.path),
        )
        .await;
    }
}
