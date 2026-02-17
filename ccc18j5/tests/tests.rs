/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "2 2 3\n", "0\n", "0\n",))
        .assert();
    assert.success().stdout(concat!("Y\n", "2\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "2 2 3\n", "0\n", "1 1\n",))
        .assert();
    assert.success().stdout(concat!("Y\n", "2\n",));
}
