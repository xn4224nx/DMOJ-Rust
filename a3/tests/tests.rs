/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "1\n",)).assert();
    assert.success().stdout(concat!("192\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "1\n", "900\n", "10000\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("192\n", "942\n", "10192\n",));
}
