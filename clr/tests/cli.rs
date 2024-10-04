use anyhow::Result;
use assert_cmd::Command;
use pretty_assertions::assert_eq;
use std::fs;

const COMMAND: &str = "clr";

#[test]
fn clr() -> Result<()> {
    let mut cmd = Command::cargo_bin(COMMAND).unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
    Ok(())
}

#[test]
fn clr_again() {
    let mut cmd = Command::cargo_bin(COMMAND).unwrap();
    let out = cmd.output().expect("fail");
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).expect("invalid utf-8");
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn clr_again_with_file() -> Result<()> {
    let file = "tests/expected/out.txt";
    let expected = fs::read_to_string(file)?;
    let mut cmd = Command::cargo_bin(COMMAND)?;
    let out = cmd.output()?;
    let stdout = String::from_utf8(out.stdout).expect("invalid utf-8");
    assert_eq!(stdout, expected);

    Ok(())
}

#[test]
fn clr_again_with_file_without_anyhow() {
    let file = "tests/expected/out.txt";
    let expected = fs::read_to_string(file).unwrap();
    let mut cmd = Command::cargo_bin(COMMAND).unwrap();
    let out = cmd.output().unwrap();
    let stdout = String::from_utf8(out.stdout).expect("invalid utf-8");
    assert_eq!(stdout, expected);
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
