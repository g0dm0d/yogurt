use std::path::PathBuf;

use yogurt::tools::file::read_file;

pub fn test_read_file() {
    assert_eq!(
        "he quick brown fox jumps over the lazy dog",
        read_file(&PathBuf::from("./tests/tools/example")).unwrap()
    );
}
