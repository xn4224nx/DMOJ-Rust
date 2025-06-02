/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dmopc14c7p2").unwrap();
    let assert = cmd.write_stdin("5\n2 1 3 5 10\n").assert();
    assert.success().stdout("9\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("dmopc14c7p2").unwrap();
    let assert = cmd.write_stdin("5\n1 2 5 4 9\n").assert();
    assert.success().stdout("unknown\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("dmopc14c7p2").unwrap();
    let assert = cmd.write_stdin("8\n6 9 2 5 4 1 7 3\n").assert();
    assert.success().stdout("unknown\n");
}
