/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci08c3p2").unwrap();
    let assert = cmd.write_stdin("zepelepenapa papapripikapa\n").assert();
    assert.success().stdout("zelena paprika\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci08c3p2").unwrap();
    let assert = cmd
        .write_stdin("bapas jepe doposapadnapa opovapa kepemipijapa\n")
        .assert();
    assert.success().stdout("bas je dosadna ova kemija\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci08c3p2").unwrap();
    let assert = cmd.write_stdin("kepemipijapa papapripikapa\n").assert();
    assert.success().stdout("kemija paprika\n");
}
