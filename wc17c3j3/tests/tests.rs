/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("wc17c3j3").unwrap();
    let assert = cmd.write_stdin("PassW0rd\n").assert();
    assert.success().stdout("Valid\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("wc17c3j3").unwrap();
    let assert = cmd.write_stdin("CorrectHorseBatteryStaple\n").assert();
    assert.success().stdout("Invalid\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("wc17c3j3").unwrap();
    let assert = cmd.write_stdin("AAAaaa1\n").assert();
    assert.success().stdout("Invalid\n");
}

#[test]
fn exp_04() {
    let mut cmd = Command::cargo_bin("wc17c3j3").unwrap();
    let assert = cmd.write_stdin("aaaAAAaaaAAA1\n").assert();
    assert.success().stdout("Invalid\n");
}
