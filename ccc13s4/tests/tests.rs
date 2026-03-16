/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("10 3\n", "8 4\n", "3 8\n", "4 2\n", "3 2\n",))
        .assert();
    assert.success().stdout(concat!("yes\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("10 3\n", "3 8\n", "2 8\n", "3 4\n", "3 2\n",))
        .assert();
    assert.success().stdout(concat!("unknown\n",));
}
