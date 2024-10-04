use assert_cmd::Command;
use predicates::str::*;

const COMMAND: &str = "gato";

#[test]
fn gato_no_args() {
    let mut cmd = Command::cargo_bin(COMMAND).unwrap();
    cmd.assert().success().stderr(contains("Failed"));
}
