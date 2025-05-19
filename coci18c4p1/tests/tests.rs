/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci18c4p1").unwrap();
    let assert = cmd.write_stdin("A\n3\nB A\nC B\nD A\n").assert();
    assert.success().stdout("C\n3\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci18c4p1").unwrap();
    let assert = cmd.write_stdin("N\n5\nD A\nN B\nB A\nC D\nF A\n").assert();
    assert.success().stdout("N\n1\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci18c4p1").unwrap();
    let assert = cmd.write_stdin("X\n4\nA X\nB X\nX A\nD A\n").assert();
    assert.success().stdout("X\n2\n");
}
