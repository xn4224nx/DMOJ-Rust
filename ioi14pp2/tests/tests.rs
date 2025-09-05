/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ioi14pp2").unwrap();
    let assert = cmd
        .write_stdin(concat!("11 3\n", "1 1 0 0 1 1 0 1 0 0 1\n",))
        .assert();
    assert.success().stdout("4\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ioi14pp2").unwrap();
    let assert = cmd
        .write_stdin(concat!("13 3\n", "1 0 0 0 1 0 0 1 0 0 0 0 1\n",))
        .assert();
    assert.success().stdout("-1\n");
}
