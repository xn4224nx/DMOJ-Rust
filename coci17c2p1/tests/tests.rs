/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("2\n", "1 10\n", "10 1\n",))
        .assert();
    assert.success().stdout(concat!("0\n", "0\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "1 1\n", "3 3\n", "3 4\n",))
        .assert();
    assert.success().stdout(concat!("0\n", "4\n", "4\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("2\n", "5 8\n", "6 4\n",)).assert();
    assert.success().stdout(concat!("8\n", "6\n",));
}
