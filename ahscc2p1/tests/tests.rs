/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("400 2 5\n", "100 100 75 50 0 100\n",))
        .assert();
    assert.success().stdout(concat!("MASTER\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("500 3 4\n", "100 100 100 0 0 0\n",))
        .assert();
    assert.success().stdout(concat!("REJECTED AGAIN\n",));
}
