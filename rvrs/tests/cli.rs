use assert_cmd::Command;

#[test]
#[ignore = "not finished"]
fn test_rvrs() {
    let mut cmd = Command::cargo_bin("rvrs").unwrap();
    cmd.assert().success().stdout("rvrs\n");
}
