/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("8\n", "10 50 40 7 3 110 90 2\n",))
        .assert();
    assert.success().stdout(concat!("10 40 7 50 3 90 2 110\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("5\n", "10 11 12 13 14\n",))
        .assert();
    assert.success().stdout(concat!("12 13 11 14 10\n",));
}
