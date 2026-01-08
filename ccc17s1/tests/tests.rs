/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "1 3 3\n", "2 2 6\n",))
        .assert();
    assert.success().stdout(concat!("2\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "1 2 3\n", "4 5 6\n",))
        .assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "1 2 3 4\n", "1 3 2 4\n",))
        .assert();
    assert.success().stdout(concat!("4\n",));
}
