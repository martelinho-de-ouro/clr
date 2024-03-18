use assert_cmd::Command;

#[test]
fn test_rvrs_failure() {
    let mut cmd = Command::cargo_bin("rvrs").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_rvrs_reverse() {
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
fn test_rvrs_reverse_no_nl() {
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
