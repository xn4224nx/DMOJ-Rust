/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("6\n", "8\n",)).assert();
    assert
        .success()
        .stdout(concat!("There are 5 ways to get the sum 10.\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("12\n", "4\n",)).assert();
    assert
        .success()
        .stdout(concat!("There are 4 ways to get the sum 10.\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("5\n", "4\n",)).assert();
    assert
        .success()
        .stdout(concat!("There are 0 ways to get the sum 10.\n",));
}
