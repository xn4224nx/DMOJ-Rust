/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("2\n", "212\n", "1253\n",)).assert();
    assert.success().stdout(concat!("1953566\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("5\n", "23\n", "17\n", "43\n", "52\n", "22\n",))
        .assert();
    assert.success().stdout(concat!("102\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "213\n", "102\n", "45\n",))
        .assert();
    assert.success().stdout(concat!("10385\n",));
}
