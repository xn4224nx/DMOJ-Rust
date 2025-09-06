/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc14s1").unwrap();
    let assert = cmd.write_stdin("10\n2\n2\n3\n").assert();
    assert.success().stdout("1\n3\n7\n9\n");
}
