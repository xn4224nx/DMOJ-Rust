/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("9\n", "4\n", "8\n",)).assert();
    assert.success().stdout(concat!("The 1-3-sum is 120\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("0\n", "5\n", "2\n",)).assert();
    assert.success().stdout(concat!("The 1-3-sum is 108\n",));
}
