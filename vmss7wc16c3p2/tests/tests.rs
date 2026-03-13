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
            "6 7 1 6\n",
            "1 2\n",
            "2 3\n",
            "2 5\n",
            "5 1\n",
            "3 4\n",
            "4 5\n",
            "4 6\n",
        ))
        .assert();
    assert.success().stdout(concat!("GO SHAHIR!\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6 6 1 6\n",
            "1 2\n",
            "2 3\n",
            "2 5\n",
            "5 1\n",
            "3 4\n",
            "4 5\n",
        ))
        .assert();
    assert.success().stdout(concat!("NO SHAHIR!\n",));
}
