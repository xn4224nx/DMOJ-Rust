/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc04j4").unwrap();
    let assert = cmd.write_stdin("ACT\nBANANA & PEEL\n").assert();
    assert.success().stdout("BCGAPTPGXL\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc04j4").unwrap();
    let assert = cmd.write_stdin("TRICKY\nI LOVE PROGRAMMING!\n").assert();
    assert.success().stdout("BCWXONKFOTKKFZVI\n");
}
