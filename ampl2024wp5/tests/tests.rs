/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("2\n", "1 2\n", "1 2\n",)).assert();
    assert.success().stdout(concat!("0 2\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "31 41 59\n", "27 18 28\n",))
        .assert();
    assert.success().stdout(concat!("1326 1886\n",));
}
