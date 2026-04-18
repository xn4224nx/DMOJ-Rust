/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("2\n", "1 2 3 4 5 6\n", "4 3 2 1 6 5\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("Twin snowflakes found.\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("2\n", "1 1 3 4 5 6\n", "4 3 2 1 6 5\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("No two snowflakes are alike.\n",));
}
