/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("5\n", "3\n", "2\n", "9\n", "20\n", "22\n", "14\n",))
        .assert();
    assert.success().stdout(concat!("19\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("10\n", "10\n", "3\n", "5\n", "13\n",))
        .assert();
    assert.success().stdout(concat!("10\n",));
}
