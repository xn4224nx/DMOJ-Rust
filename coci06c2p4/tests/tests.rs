/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin("coci06c2p4").unwrap();
    let assert = cmd.write_stdin("3\n").assert();
    assert.success().stdout("0\n");
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin("coci06c2p4").unwrap();
    let assert = cmd.write_stdin("4\n").assert();
    assert.success().stdout("1\n");
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin("coci06c2p4").unwrap();
    let assert = cmd.write_stdin("5\n").assert();
    assert.success().stdout("5\n");
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin("coci06c2p4").unwrap();
    let assert = cmd.write_stdin("6\n").assert();
    assert.success().stdout("15\n");
}
