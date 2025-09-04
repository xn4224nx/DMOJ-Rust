/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc14j2").unwrap();
    let assert = cmd.write_stdin("6\nABBABB\n").assert();
    assert.success().stdout("B\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc14j2").unwrap();
    let assert = cmd.write_stdin("6\nABBABA\n").assert();
    assert.success().stdout("Tie\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc14j2").unwrap();
    let assert = cmd.write_stdin("6\nABBAAA\n").assert();
    assert.success().stdout("A\n");
}
