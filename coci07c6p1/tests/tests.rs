/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci07c6p1").unwrap();
    let assert = cmd.write_stdin("5 3 1\n1 6\n3 5\n2 8\n").assert();
    assert.success().stdout("33\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci07c6p1").unwrap();
    let assert = cmd.write_stdin("10 8 6\n15 30\n25 50\n70 80\n").assert();
    assert.success().stdout("480\n");
}
