use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// Verify sha1 sum of file
pub fn verify_sha1sum(path: &PathBuf, expected_sha1sum: &String) -> bool {
    if expected_sha1sum == "" {
        return true;
    }

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let mut buf = Vec::new();
    match file.read_to_end(&mut buf) {
        Err(why) => panic!("couldn't read file {}", why),
        Ok(_) => {}
    }

    let mut hasher = Sha1::new();
    hasher.update(&buf);
    let actual_sha1sum = format!("{:x}", hasher.finalize());

    if actual_sha1sum != *expected_sha1sum {
        println!(
            "Sha sum not eq actual: {} , expect: {}",
            actual_sha1sum, expected_sha1sum
        );
        return false;
    }
    return true;
}
