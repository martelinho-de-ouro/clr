use assert_cmd::Command;
use predicates::str::*;

#[test]
fn test_gato_error_no_args() {
    let mut cmd = Command::cargo_bin("gato").unwrap();
    cmd.assert().success().stdout(contains("file"));
}
