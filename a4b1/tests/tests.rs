/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("a4b1").unwrap();
    let assert = cmd.write_stdin("4\n4\n3\n2\n1\n").assert();
    assert.success().stdout("1\n2\n3\n4\n");
}
