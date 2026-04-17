/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1 2 3 4\n",)).assert();
    assert
        .success()
        .stdout(concat!("Go to the department store\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("72 7 42 0\n",)).assert();
    assert.success().stdout(concat!("Stay home\n",));
}
