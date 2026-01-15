/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("4\n", "17\n",)).assert();
    assert.success().stdout(concat!("13\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("6\n", "10\n",)).assert();
    assert
        .success()
        .stdout(concat!("No such integer exists.\n",));
}
