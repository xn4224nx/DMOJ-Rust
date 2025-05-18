/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci12c5p1").unwrap();
    let assert = cmd.write_stdin("AEB|C\n").assert();
    assert.success().stdout("C-dur\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci12c5p1").unwrap();
    let assert = cmd
        .write_stdin("CD|EC|CD|EC|EF|G|EF|G|GAGF|EC|GAGF|EC|CG|C|CG|C\n")
        .assert();
    assert.success().stdout("C-dur\n");
}
