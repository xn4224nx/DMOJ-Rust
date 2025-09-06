/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc20j1").unwrap();
    let assert = cmd.write_stdin("3\n1\n0\n").assert();
    assert.success().stdout("sad\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc20j1").unwrap();
    let assert = cmd.write_stdin("3\n2\n1\n").assert();
    assert.success().stdout("happy\n");
}
