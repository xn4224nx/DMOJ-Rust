/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("5\n", "1 0 1 0 1\n", "0 0 0 0 0\n",))
        .assert();
    assert.success().stdout(concat!("9\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("7\n", "0 0 1 1 0 0 0\n", "0 0 1 0 1 0 0\n",))
        .assert();
    assert.success().stdout(concat!("8\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("2\n", "1 1\n", "1 1\n",)).assert();
    assert.success().stdout(concat!("6\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("7\n", "0 0 0 1 0 0 0\n", "0 0 0 1 0 0 0\n",))
        .assert();
    assert.success().stdout(concat!("6\n",));
}
