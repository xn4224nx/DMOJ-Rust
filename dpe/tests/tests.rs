/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3 8\n", "3 30\n", "4 50\n", "5 60\n",))
        .assert();
    assert.success().stdout(concat!("90\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("1 1000000000\n", "1000000000 10\n",))
        .assert();
    assert.success().stdout(concat!("10\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6 15\n", "6 5\n", "5 6\n", "6 4\n", "6 6\n", "3 5\n", "7 2\n",
        ))
        .assert();
    assert.success().stdout(concat!("17\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("0 1000000000\n",)).assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3 7\n", "1 1\n", "3 4\n", "4 5\n", "5 7\n",))
        .assert();
    assert.success().stdout(concat!("9\n",));
}

#[test]
fn full_program_exp5() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "10 67\n", "23 505\n", "26 352\n", "20 458\n", "18 220\n", "32 354\n", "27 414\n",
            "29 498\n", "26 545\n", "30 473\n", "27 543\n",
        ))
        .assert();
    assert.success().stdout(concat!("1270\n",));
}

#[test]
fn full_program_exp6() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6 0\n", "6 5\n", "5 6\n", "6 4\n", "6 6\n", "3 5\n", "7 2\n",
        ))
        .assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp7() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6 3\n", "6 5\n", "5 6\n", "6 4\n", "6 6\n", "3 5\n", "7 2\n",
        ))
        .assert();
    assert.success().stdout(concat!("5\n",));
}

#[test]
fn full_program_exp8() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6 2\n", "6 5\n", "5 6\n", "6 4\n", "6 6\n", "3 5\n", "7 2\n",
        ))
        .assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp9() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3 50\n", "10 60\n", "20 100\n", "30 120\n",))
        .assert();
    assert.success().stdout(concat!("220\n",));
}

#[test]
fn full_program_exp10() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "9 15\n", "3 2\n", "5 3\n", "7 3\n", "4 4\n", "3 4\n", "9 5\n", "2 7\n", "11 8\n",
            "5 8\n",
        ))
        .assert();
    assert.success().stdout(concat!("23\n",));
}
