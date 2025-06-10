/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc13s1").unwrap();
    let assert = cmd.write_stdin("1987\n").assert();
    assert.success().stdout("2013\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc13s1").unwrap();
    let assert = cmd.write_stdin("999\n").assert();
    assert.success().stdout("1023\n");
}
