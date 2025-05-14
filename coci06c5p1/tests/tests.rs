/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci06c5p1").unwrap();
    let assert = cmd.write_stdin("AB\n").assert();
    assert.success().stdout("3\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci06c5p1").unwrap();
    let assert = cmd.write_stdin("CBABCACCC\n").assert();
    assert.success().stdout("1\n");
}
