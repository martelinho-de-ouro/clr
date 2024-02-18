use assert_cmd::Command;

// use std::process::Command;
//   use assert_cmd::Command;

#[test]
fn test_runs() {
    let mut cmd = Command::cargo_bin("clr").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
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

// #[test]
// fn test_args() {
//     let mut cmd = Command::cargo_bin("args").unwrap();
//     cmd.assert().success().stdout("Args { inner: [\"target/debug/args\"] }").get_output().;
// }
