/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("l 2\n", "d 2\n", "r 1\n", "q 0\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("-3 -5 safe\n", "-3 -7 safe\n", "-2 -7 safe\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("r 2\n", "d 10\n", "r 4\n", "q 0\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("1 -5 safe\n", "1 -15 DANGER\n",));
}
