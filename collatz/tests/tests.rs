/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("7\n",)).assert();
    assert.success().stdout(concat!("16\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("27\n",)).assert();
    assert.success().stdout(concat!("111\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("97\n",)).assert();
    assert.success().stdout(concat!("118\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("871\n",)).assert();
    assert.success().stdout(concat!("178\n",));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("6171\n",)).assert();
    assert.success().stdout(concat!("261\n",));
}

#[test]
fn full_program_exp5() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("77031\n",)).assert();
    assert.success().stdout(concat!("350\n",));
}

#[test]
fn full_program_exp6() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("837799\n",)).assert();
    assert.success().stdout(concat!("524\n",));
}

#[test]
fn full_program_exp7() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("8400511\n",)).assert();
    assert.success().stdout(concat!("685\n",));
}

#[test]
fn full_program_exp8() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("63728127\n",)).assert();
    assert.success().stdout(concat!("949\n",));
}

#[test]
fn full_program_exp9() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n",)).assert();
    assert.success().stdout(concat!("0\n",));
}
