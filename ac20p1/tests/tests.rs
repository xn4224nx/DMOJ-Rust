/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("2\n", "5 12 13\n", "5 11 13\n",))
        .assert();
    assert.success().stdout(concat!("R\n", "O\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "2 3 4\n", "2 2 2\n", "3 4 5\n",))
        .assert();
    assert.success().stdout(concat!("O\n", "A\n", "R\n",));
}
