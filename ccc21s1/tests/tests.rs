/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "2 3 6 2\n", "4 1 1\n",))
        .assert();
    assert.success().stdout(concat!("18.5\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "6 4 9 7 3\n", "5 2 4 1\n",))
        .assert();
    assert.success().stdout(concat!("75\n",));
}
