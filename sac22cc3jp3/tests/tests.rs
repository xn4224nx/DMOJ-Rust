/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "A\n", "B\n", "B\n",))
        .assert();
    assert.success().stdout(concat!("0.640000\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "B\n", "C\n", "D\n", "E\n",))
        .assert();
    assert.success().stdout(concat!("0.038400\n",));
}
