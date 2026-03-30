/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("4\n", "10 30 40 20\n",)).assert();
    assert.success().stdout(concat!("30\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("2\n", "10 10\n",)).assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("6\n", "30 10 60 10 60 50\n",))
        .assert();
    assert.success().stdout(concat!("40\n",));
}
