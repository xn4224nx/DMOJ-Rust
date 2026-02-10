/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("28\n", "7\n",)).assert();
    assert.success().stdout(concat!("4\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("13\n", "5\n",)).assert();
    assert.success().stdout(concat!("2 3/5\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("0\n", "7\n",)).assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("55\n", "10\n",)).assert();
    assert.success().stdout(concat!("5 1/2\n",));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("10\n", "12\n",)).assert();
    assert.success().stdout(concat!("5/6\n",));
}

#[test]
fn full_program_exp5() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("12\n", "10\n",)).assert();
    assert.success().stdout(concat!("1 1/5\n",));
}

#[test]
fn full_program_exp6() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("20\n", "24\n",)).assert();
    assert.success().stdout(concat!("5/6\n",));
}
