use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_gato_error_no_args() {
    let mut cmd = Command::cargo_bin("gato").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("file"));
}
