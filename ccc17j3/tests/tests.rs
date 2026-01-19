/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("3 4\n", "3 3\n", "3\n",)).assert();
    assert.success().stdout(concat!("Y\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("10 2\n", "10 4\n", "5\n",))
        .assert();
    assert.success().stdout(concat!("N\n",));
}
