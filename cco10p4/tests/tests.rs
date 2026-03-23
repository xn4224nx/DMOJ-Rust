/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2\n",
            "5\n",
            "10 6 1\n",
            "5 7 1\n",
            "6 10 2\n",
            "1 5 1\n",
            "11 11 2\n",
            "16\n",
        ))
        .assert();
    assert.success().stdout(concat!("18\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2\n",
            "5\n",
            "10 6 2\n",
            "5 7 2\n",
            "6 10 2\n",
            "1 5 2\n",
            "11 11 2\n",
            "16\n",
        ))
        .assert();
    assert.success().stdout(concat!("-1\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2\n",
            "5\n",
            "10 6 2\n",
            "50 7 1\n",
            "6 10 2\n",
            "1 5 2\n",
            "11 11 2\n",
            "16\n",
        ))
        .assert();
    assert.success().stdout(concat!("-1\n",));
}
