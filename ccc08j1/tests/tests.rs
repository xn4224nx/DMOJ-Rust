/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc08j1").unwrap();
    let assert = cmd.write_stdin("69\n1.73\n").assert();
    assert.success().stdout("Normal weight\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc08j1").unwrap();
    let assert = cmd.write_stdin("84.5\n1.8\n").assert();
    assert.success().stdout("Overweight\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc08j1").unwrap();
    let assert = cmd.write_stdin("44.7\n2.1\n").assert();
    assert.success().stdout("Underweight\n");
}
