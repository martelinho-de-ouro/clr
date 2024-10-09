use assert_cmd::Command;
use predicates::str::*;

const COMMAND: &str = "bobblehead";

#[test]
fn bobblehead() {
    let mut cmd = Command::cargo_bin(COMMAND).unwrap();
    cmd.arg("bh.txt").assert().stdout(contains("_"));
}

