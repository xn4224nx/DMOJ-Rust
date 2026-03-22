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
            "10\n", "3\n", "4\n", "8 0\n", "1 1\n", "4 2\n", "4\n", "10\n", "7\n", "1\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("Y\n", "N\n", "N\n", "Y\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "10\n", "3\n", "10\n", "8 0\n", "1 1\n", "4 2\n", "1\n", "2\n", "3\n", "4\n", "5\n",
            "6\n", "7\n", "8\n", "9\n", "10\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "Y\n", "Y\n", "Y\n", "Y\n", "Y\n", "Y\n", "N\n", "Y\n", "N\n", "N\n",
    ));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "10\n", "2\n", "10\n", "10 3\n", "1 1\n", "1\n", "2\n", "3\n", "4\n", "5\n", "6\n",
            "7\n", "8\n", "9\n", "10\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "Y\n", "Y\n", "N\n", "N\n", "N\n", "N\n", "Y\n", "Y\n", "Y\n", "Y\n",
    ));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "10\n", "1\n", "10\n", "9 2\n", "1\n", "2\n", "3\n", "4\n", "5\n", "6\n", "7\n", "8\n",
            "9\n", "10\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "N\n", "N\n", "N\n", "N\n", "N\n", "N\n", "Y\n", "Y\n", "Y\n", "Y\n",
    ));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "10\n", "1\n", "10\n", "9 1\n", "1\n", "2\n", "3\n", "4\n", "5\n", "6\n", "7\n", "8\n",
            "9\n", "10\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "N\n", "N\n", "N\n", "N\n", "N\n", "N\n", "N\n", "Y\n", "Y\n", "Y\n",
    ));
}

#[test]
fn full_program_exp5() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "10\n", "1\n", "10\n", "9 0\n", "1\n", "2\n", "3\n", "4\n", "5\n", "6\n", "7\n", "8\n",
            "9\n", "10\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "N\n", "N\n", "N\n", "N\n", "N\n", "N\n", "N\n", "N\n", "Y\n", "N\n",
    ));
}
