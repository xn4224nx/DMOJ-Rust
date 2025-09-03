/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci13c5p2").unwrap();
    let assert = cmd.write_stdin(concat!("2\n", "2 1 3\n")).assert();
    assert.success().stdout(concat!("1\n", "2 3\n"));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci13c5p2").unwrap();
    let assert = cmd.write_stdin(concat!("3\n", "1 6 4 3 5 2 7\n")).assert();
    assert
        .success()
        .stdout(concat!("3\n", "6 2\n", "1 4 5 7\n"));
}
