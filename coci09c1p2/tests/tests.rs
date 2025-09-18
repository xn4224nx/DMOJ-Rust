/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci09c1p2").unwrap();
    let assert = cmd.write_stdin("2\n").assert();
    assert.success().stdout("12\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci09c1p2").unwrap();
    let assert = cmd.write_stdin("3\n").assert();
    assert.success().stdout("30\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci09c1p2").unwrap();
    let assert = cmd.write_stdin("15\n").assert();
    assert.success().stdout("2040\n");
}
