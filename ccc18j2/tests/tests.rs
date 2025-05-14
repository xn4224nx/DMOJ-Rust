/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc18j2").unwrap();
    let assert = cmd.write_stdin("5\nCC..C\n.CC..\n").assert();
    assert.success().stdout("1\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc18j2").unwrap();
    let assert = cmd.write_stdin("7\nCCCCCCC\nC.C.C.C\n").assert();
    assert.success().stdout("4\n");
}
