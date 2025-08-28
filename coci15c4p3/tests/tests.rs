/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci15c4p3").unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "0 1 1\n", "1 0 1\n", "1 1 0\n",))
        .assert();
    assert.success().stdout(concat!("1 1 1\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci15c4p3").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n",
            "0 0 1 1 1\n",
            "0 0 2 0 2\n",
            "1 2 0 1 3\n",
            "1 0 1 0 1\n",
            "1 2 3 1 0\n",
        ))
        .assert();
    assert.success().stdout(concat!("1 2 3 1 11\n",));
}
