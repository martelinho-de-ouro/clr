use assert_cmd::Command;

#[test]
fn test_depsdot_failure() {
    let mut cmd = Command::cargo_bin("depsdot").unwrap();
    cmd.assert().failure();
}