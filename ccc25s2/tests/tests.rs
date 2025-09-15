/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc25s2").unwrap();
    let assert = cmd.write_stdin("r2d2\n8\n").assert();
    assert.success().stdout("r\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc25s2").unwrap();
    let assert = cmd.write_stdin("a4b1c2d10\n100\n").assert();
    assert.success().stdout("d\n");
}
