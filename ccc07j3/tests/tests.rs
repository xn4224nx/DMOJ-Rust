/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc07j3").unwrap();
    let assert = cmd.write_stdin("2\n3\n8\n198000\n").assert();
    assert.success().stdout("no deal\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc07j3").unwrap();
    let assert = cmd
        .write_stdin("8\n10\n9\n8\n7\n6\n5\n4\n3\n400\n")
        .assert();
    assert.success().stdout("deal\n");
}
