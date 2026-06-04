/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("5 1\n", "1 2\n",)).assert();
    assert.success().stdout(concat!("2\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("6 2\n", "1 4\n", "1 2\n",))
        .assert();
    assert.success().stdout(concat!("1\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("10 3\n", "1 2\n", "2 3\n", "3 2\n",))
        .assert();
    assert.success().stdout(concat!("2\n",));
}
