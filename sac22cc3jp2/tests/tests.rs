/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("7 -2\n", "-3 -2\n",)).assert();
    assert.success().stdout(concat!("x-axis\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("-5 5\n", "-5 10\n",)).assert();
    assert.success().stdout(concat!("y-axis\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("3 3\n", "-3 -3\n",)).assert();
    assert.success().stdout(concat!("neither\n",));
}
