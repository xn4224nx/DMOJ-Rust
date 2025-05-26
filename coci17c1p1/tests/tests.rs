/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci17c1p1").unwrap();
    let assert = cmd.write_stdin("6\n2\n3\n2\n3\n2\n3\n").assert();
    assert.success().stdout("DOSTA\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci17c1p1").unwrap();
    let assert = cmd.write_stdin("1\n10\n").assert();
    assert.success().stdout("VUCI\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci17c1p1").unwrap();
    let assert = cmd.write_stdin("2\n5\n6\n").assert();
    assert.success().stdout("VUCI\n");
}
