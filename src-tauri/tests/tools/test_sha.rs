use std::path::PathBuf;

use yogurt::tools::sha::verify_sha1sum;

#[test]
fn test_verify_sha1sum() {
    assert!(
        verify_sha1sum(
            &PathBuf::from("./tests/tools/example"),
            "a9119862a50e722145372c753b7baf9d74694f71",
        )
        .unwrap()
    );

    assert!(
        verify_sha1sum(&PathBuf::from("./tests/tools/example"), "",).unwrap()
    );

    assert!(
        !verify_sha1sum(&PathBuf::from("./tests/tools/example"), "test",).unwrap()
    );
}
