use assert_cmd::Command;
use predicates::str::*;

const COMMAND: &str = "gato";

#[test]
#[ignore]
fn gato_no_args() {
    let mut cmd = Command::cargo_bin(COMMAND).unwrap();
    cmd.assert().failure();
}
