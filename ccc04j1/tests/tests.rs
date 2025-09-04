/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc04j1").unwrap();
    let assert = cmd.write_stdin("9\n").assert();
    assert
        .success()
        .stdout("The largest square has side length 3.\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc04j1").unwrap();
    let assert = cmd.write_stdin("8\n").assert();
    assert
        .success()
        .stdout("The largest square has side length 2.\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc04j1").unwrap();
    let assert = cmd.write_stdin("7535\n").assert();
    assert
        .success()
        .stdout("The largest square has side length 86.\n");
}
