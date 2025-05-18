/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci13c3p1").unwrap();
    let assert = cmd.write_stdin("0\n").assert();
    assert.success().stdout("1 0\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci13c3p1").unwrap();
    let assert = cmd.write_stdin("1\n").assert();
    assert.success().stdout("0 1\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci13c3p1").unwrap();
    let assert = cmd.write_stdin("2\n").assert();
    assert.success().stdout("1 1\n");
}

#[test]
fn exp_04() {
    let mut cmd = Command::cargo_bin("coci13c3p1").unwrap();
    let assert = cmd.write_stdin("3\n").assert();
    assert.success().stdout("1 2\n");
}

#[test]
fn exp_05() {
    let mut cmd = Command::cargo_bin("coci13c3p1").unwrap();
    let assert = cmd.write_stdin("4\n").assert();
    assert.success().stdout("2 3\n");
}

#[test]
fn exp_06() {
    let mut cmd = Command::cargo_bin("coci13c3p1").unwrap();
    let assert = cmd.write_stdin("10\n").assert();
    assert.success().stdout("34 55\n");
}
