/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "7 9 5\n", "6 13 10\n",))
        .assert();
    assert.success().stdout(concat!("DA\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "5 3 3 5\n", "10 2 10 10\n",))
        .assert();
    assert.success().stdout(concat!("NE\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "5 2 3 2\n", "3 8 3 3\n",))
        .assert();
    assert.success().stdout(concat!("DA\n",));
}
