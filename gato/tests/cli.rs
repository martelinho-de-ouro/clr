use assert_cmd::Command;
use predicates::str::*;

#[test]
fn gato_error_no_args() {
    let mut cmd = Command::cargo_bin("gato").unwrap();
    cmd.assert().success().stdout(contains("file"));
}
