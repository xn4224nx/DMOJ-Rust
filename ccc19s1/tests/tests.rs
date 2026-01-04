/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("HV\n",)).assert();
    assert.success().stdout(concat!("4 3\n", "2 1\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("VVHH\n",)).assert();
    assert.success().stdout(concat!("1 2\n", "3 4\n",));
}
