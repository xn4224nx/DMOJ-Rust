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
            "2 S\n", "TH\n", "9C\n", "KS\n", "QS\n", "JS\n", "TD\n", "AD\n", "JH\n",
        ))
        .assert();
    assert.success().stdout(concat!("60\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4 H\n", "AH\n", "KH\n", "QH\n", "JH\n", "TH\n", "9H\n", "8H\n", "7H\n", "AS\n",
            "KS\n", "QS\n", "JS\n", "TS\n", "9S\n", "8S\n", "7S\n",
        ))
        .assert();
    assert.success().stdout(concat!("92\n",));
}
