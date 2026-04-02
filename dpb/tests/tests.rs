/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("5 3\n", "10 30 40 50 20\n",))
        .assert();
    assert.success().stdout(concat!("30\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("3 1\n", "10 20 10\n",)).assert();
    assert.success().stdout(concat!("20\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("2 100\n", "10 10\n",)).assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("10 4\n", "40 10 20 70 80 10 20 70 80 60\n",))
        .assert();
    assert.success().stdout(concat!("40\n",));
}
