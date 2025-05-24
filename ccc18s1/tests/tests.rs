/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc18s1").unwrap();
    let assert = cmd.write_stdin("5\n16\n0\n10\n4\n15\n").assert();
    assert.success().stdout("3.0\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc18s1").unwrap();
    let assert = cmd.write_stdin("6\n20\n50\n4\n19\n15\n1\n").assert();
    assert.success().stdout("2.5\n");
}
