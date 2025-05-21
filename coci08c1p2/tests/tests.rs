/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci08c1p2").unwrap();
    let assert = cmd.write_stdin("5\nBAACC\n").assert();
    assert.success().stdout("3\nBruno\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci08c1p2").unwrap();
    let assert = cmd.write_stdin("9\nAAAABBBBB\n").assert();
    assert.success().stdout("4\nAdrian\nBruno\nGoran\n");
}
