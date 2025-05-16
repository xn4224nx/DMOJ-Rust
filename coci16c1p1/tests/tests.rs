/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci16c1p1").unwrap();
    let assert = cmd.write_stdin("10\n3\n4\n6\n2\n").assert();
    assert.success().stdout("28\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci16c1p1").unwrap();
    let assert = cmd.write_stdin("10\n3\n10\n2\n12\n").assert();
    assert.success().stdout("16\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci16c1p1").unwrap();
    let assert = cmd.write_stdin("15\n3\n15\n10\n20\n").assert();
    assert.success().stdout("15\n");
}
