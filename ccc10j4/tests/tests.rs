/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "7 3 4 6 4 5 7 5\n",
            "3 1 3 5\n",
            "3 1 4 5\n",
            "4 3 4 6 7\n",
            "0\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("3\n", "1\n", "2\n", "2\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("7 3 4 6 4 5 7 5\n", "0\n",))
        .assert();
    assert.success().stdout(concat!("3\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("3 1 3 5\n", "0\n",)).assert();
    assert.success().stdout(concat!("1\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("3 1 4 5\n", "0\n",)).assert();
    assert.success().stdout(concat!("2\n",));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("4 3 4 6 7\n", "0\n",)).assert();
    assert.success().stdout(concat!("2\n",));
}

#[test]
fn full_program_exp5() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1 4\n", "0\n",)).assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp6() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("10 4 5 6 7 8 9 10 11 12 13\n", "0\n",))
        .assert();
    assert.success().stdout(concat!("1\n",));
}

#[test]
fn full_program_exp7() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("10 4 5 7 10 14 19 25 40 70 90\n", "0\n",))
        .assert();
    assert.success().stdout(concat!("9\n",));
}
