/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dmopc19c3p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("10\n", "9 2 9 6 8 7 1 3 9 6\n",))
        .assert();
    assert.success().stdout(concat!("9\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("dmopc19c3p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("10\n", "9 2 9 6 8 2 7 1 3 9 6 2\n",))
        .assert();
    assert.success().stdout(concat!("2 9\n",));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("dmopc19c3p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("10\n", "-9 2 -9 6 8 2 7 1 3 -9 6 2\n",))
        .assert();
    assert.success().stdout(concat!("-9 2\n",));
}
