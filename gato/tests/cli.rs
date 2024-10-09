use assert_cmd::Command;
use predicates::str::*;

const COMMAND: &str = "gato";

#[test]
fn gato() {
    let mut cmd = Command::cargo_bin(COMMAND).unwrap();
    cmd.arg("Cargo.toml").assert().stdout(contains("👏"));
}
