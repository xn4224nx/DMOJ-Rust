/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "10 40 70\n", "20 50 80\n", "30 60 90\n",))
        .assert();
    assert.success().stdout(concat!("210\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "100 10 1\n",)).assert();
    assert.success().stdout(concat!("100\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "7\n", "6 7 8\n", "8 8 3\n", "2 5 2\n", "7 8 6\n", "4 6 8\n", "2 3 4\n", "7 5 1\n",
        ))
        .assert();
    assert.success().stdout(concat!("46\n",));
}
