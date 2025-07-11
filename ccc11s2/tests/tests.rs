/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc11s2").unwrap();
    let assert = cmd.write_stdin("3\nA\nB\nC\nA\nC\nB\n").assert();
    assert.success().stdout("1\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc11s2").unwrap();
    let assert = cmd.write_stdin("3\nA\nA\nA\nA\nB\nA\n").assert();
    assert.success().stdout("2\n");
}
