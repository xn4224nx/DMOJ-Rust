/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("2 1\n", "5 6\n", "2 1\n",))
        .assert();
    assert.success().stdout(concat!("6\n", "1\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4 2\n", "5 5 5 5\n", "1 2 1 1\n",))
        .assert();
    assert.success().stdout(concat!("15\n", "1\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4 10000000\n", "1 2 3 4\n", "2 1 4 3\n",))
        .assert();
    assert.success().stdout(concat!("4\n", "4\n",));
}
