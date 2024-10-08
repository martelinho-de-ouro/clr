use assert_cmd::Command;

const COMMAND: &str = "bobblehead";

#[test]
#[ignore]
fn rvrs_failure() {
    let mut cmd = Command::cargo_bin(COMMAND).unwrap();
    cmd.assert().failure();
}
