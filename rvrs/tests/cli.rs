use assert_cmd::Command;
use predicates::str::*;

#[test]
fn rvrs_failure() {
    let mut cmd = Command::cargo_bin("rvrs").unwrap();
    cmd.assert().failure();
}

#[test]
fn error_no_args() {
    let mut cmd = Command::cargo_bin("rvrs").unwrap();
    cmd.assert().failure().stderr(contains("Usage"));
}

#[test]
fn rvrs_reverse() {
    let mut cmd = Command::cargo_bin("rvrs").unwrap();

    let expected = "ccc bbb aaa\n";
    // https://docs.rs/assert_cmd/latest/assert_cmd/#examples
    cmd.arg("aaa")
        .arg("bbb")
        .arg("ccc")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn rvrs_reverse_no_new_line() {
    let mut cmd = Command::cargo_bin("rvrs").unwrap();

    let expected = "ccc bbb aaa";
    cmd.arg("aaa")
        .arg("bbb")
        .arg("ccc")
        .arg("-n")
        .assert()
        .success()
        .stdout(expected);
}
