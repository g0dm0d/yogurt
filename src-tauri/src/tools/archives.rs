use flate2::read::GzDecoder;
use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};
use tar::Archive;
use zip::ZipArchive;

use super::path::get_path;

pub fn unzip(path: PathBuf) {
    let file = File::open(path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    let output_folder = &get_path("java");
    if !output_folder.exists() {
        fs::create_dir(output_folder).unwrap();
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let output_path = output_folder.join(file.name());
        if file.is_dir() {
            fs::create_dir_all(&output_path).unwrap();
        } else {
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).unwrap();
                }
            }
            let mut output_file = File::create(output_path).unwrap();
            io::copy(&mut file, &mut output_file).unwrap();
        }
    }
}

pub fn untar(path: PathBuf) {
    let tar_gz = File::open(path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&get_path("java")).unwrap();
}
