/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc00s2").unwrap();
    let assert = cmd
        .write_stdin("3\n10\n20\n30\n99\n1\n50\n88\n3\n88\n2\n77\n")
        .assert();
    assert.success().stdout("5 55\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc00s2").unwrap();
    let assert = cmd.write_stdin("1\n3\n99\n1\n50\n77\n").assert();
    assert.success().stdout("2 2\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc00s2").unwrap();
    let assert = cmd.write_stdin("3\n10\n20\n30\n88\n1\n77\n").assert();
    assert.success().stdout("30 30\n");
}
