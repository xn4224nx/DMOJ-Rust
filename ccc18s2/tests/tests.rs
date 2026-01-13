/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("2\n", "1 3\n", "2 9\n",)).assert();
    assert.success().stdout(concat!("1 3\n", "2 9\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "4 3 1\n", "6 5 2\n", "9 7 3\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("1 2 3\n", "3 5 7\n", "4 6 9\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "3 7 9\n", "2 5 6\n", "1 3 4\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("1 2 3\n", "3 5 7\n", "4 6 9\n",));
}
