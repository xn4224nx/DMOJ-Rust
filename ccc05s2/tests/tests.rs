/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "100 200\n",
            "10 40\n",
            "-5 15\n",
            "30 -30\n",
            "0 0\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("10 40\n", "5 55\n", "35 25\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("30 40\n", "30 40\n", "-100 -10\n", "0 0\n",))
        .assert();
    assert.success().stdout(concat!("30 40\n", "0 30\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("1 2\n", "1 1\n", "0 1\n", "0 0\n",))
        .assert();
    assert.success().stdout(concat!("1 1\n", "1 2\n",));
}
