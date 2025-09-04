/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc16j1").unwrap();
    let assert = cmd.write_stdin("W\nL\nW\nW\nL\nW\n").assert();
    assert.success().stdout("2\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc16j1").unwrap();
    let assert = cmd.write_stdin("L\nL\nL\nL\nL\nL\n").assert();
    assert.success().stdout("-1\n");
}
