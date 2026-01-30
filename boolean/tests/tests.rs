/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("not not True\n",)).assert();
    assert.success().stdout(concat!("True\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("not not not False\n",)).assert();
    assert.success().stdout(concat!("True\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("not not not True\n",)).assert();
    assert.success().stdout(concat!("False\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("True\n",)).assert();
    assert.success().stdout(concat!("True\n",));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("False\n",)).assert();
    assert.success().stdout(concat!("False\n",));
}
