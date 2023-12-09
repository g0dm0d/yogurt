use sha1_smol::Sha1;
use std::path::PathBuf;

use super::file::read_file_binary;

/// Verify sha1 sum of file
pub fn verify_sha1sum(path: &PathBuf, expected_sha1sum: &str) -> Result<bool, String> {
    if expected_sha1sum.is_empty() {
        return Ok(true);
    }

    let file = read_file_binary(path)?;

    let mut hasher = Sha1::new();
    hasher.update(&file);
    let actual_sha1sum = hasher.digest().to_string();

    if actual_sha1sum != expected_sha1sum {
        return Ok(false);
    }
    Ok(true)
}
