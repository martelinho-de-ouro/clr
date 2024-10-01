use anyhow::Result;
use assert_cmd::Command;
use std::fs;

#[test]
fn test_clr() -> Result<()> {
    let mut cmd = Command::cargo_bin("clr").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
    Ok(())
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
fn test_clr_again_with_file() -> Result<()> {
    let file = "tests/expected/out.txt";
    let expected = fs::read_to_string(file)?;
    let mut cmd = Command::cargo_bin("clr")?;
    let out = cmd.output()?;
    let stdout = String::from_utf8(out.stdout).expect("invalid utf-8");
    assert_eq!(stdout, expected);

    Ok(())
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
