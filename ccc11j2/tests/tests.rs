/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("30\n", "10\n",)).assert();
    assert.success().stdout(concat!(
        "The balloon first touches ground at hour:\n",
        "6\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("70\n", "10\n",)).assert();
    assert.success().stdout(concat!(
        "The balloon does not touch ground in the given time.\n",
    ));
}
