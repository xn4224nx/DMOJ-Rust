/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci18c3p1").unwrap();
    let assert = cmd.write_stdin("MAGNUS\n").assert();
    assert.success().stdout("0\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci18c3p1").unwrap();
    let assert = cmd.write_stdin("HHHHOOOONNNNIIII\n").assert();
    assert.success().stdout("1\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci18c3p1").unwrap();
    let assert = cmd.write_stdin("PROHODNIHODNIK\n").assert();
    assert.success().stdout("2\n");
}
