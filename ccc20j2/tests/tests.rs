/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc20j2").unwrap();
    let assert = cmd.write_stdin("750\n1\n5\n").assert();
    assert.success().stdout("4\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc20j2").unwrap();
    let assert = cmd.write_stdin("10\n2\n1\n").assert();
    assert.success().stdout("5\n");
}
