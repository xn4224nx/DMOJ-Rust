/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("5\n", "1\n", "1\n", "1\n", "4\n", "3\n",))
        .assert();
    assert.success().stdout(concat!("3\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "10\n", "6\n", "1\n", "8\n",))
        .assert();
    assert.success().stdout(concat!("9\n",));
}
