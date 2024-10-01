use assert_cmd::Command;

#[test]
fn test_clr() {
    let mut cmd = Command::cargo_bin("clr").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn test_clr_again() {
    let mut cmd = Command::cargo_bin("clr").unwrap();
    let out = cmd.output().expect("fail");
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).expect("invalid utf-8");
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn test_true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn test_false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
