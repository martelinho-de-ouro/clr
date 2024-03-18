use assert_cmd::Command;

#[test]
fn test_depsdot_failure() {
    let mut cmd = Command::cargo_bin("depsdot").unwrap();
    cmd.assert().success();
}

#[test]
fn test_depsdot() {
    let mut cmd = Command::cargo_bin("depsdot").unwrap();
    let expected = "[\"./Cargo.toml\"]\n";
    cmd.assert().success().stdout(expected);
}
