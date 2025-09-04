/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc17j1").unwrap();
    let assert = cmd.write_stdin("12\n5\n").assert();
    assert.success().stdout("1\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc17j1").unwrap();
    let assert = cmd.write_stdin("9\n-13\n").assert();
    assert.success().stdout("4\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc17j1").unwrap();
    let assert = cmd.write_stdin("12\n-5\n").assert();
    assert.success().stdout("4\n");
}

#[test]
fn exp_04() {
    let mut cmd = Command::cargo_bin("ccc17j1").unwrap();
    let assert = cmd.write_stdin("-12\n-5\n").assert();
    assert.success().stdout("3\n");
}

#[test]
fn exp_05() {
    let mut cmd = Command::cargo_bin("ccc17j1").unwrap();
    let assert = cmd.write_stdin("-12\n5\n").assert();
    assert.success().stdout("2\n");
}
