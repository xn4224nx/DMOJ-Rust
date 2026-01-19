/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "3\n", "2\n", "R 1\n", "C 1\n",))
        .assert();
    assert.success().stdout(concat!("4\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n", "5\n", "7\n", "R 3\n", "C 1\n", "C 2\n", "R 2\n", "R 2\n", "C 1\n", "R 4\n",
        ))
        .assert();
    assert.success().stdout(concat!("10\n",));
}
