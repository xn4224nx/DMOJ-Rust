/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "0 100\n", "20 50\n", "10 120\n",))
        .assert();
    assert.success().stdout(concat!("7.0\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n", "20 -5\n", "0 -17\n", "10 31\n", "5 -3\n", "30 11\n",
        ))
        .assert();
    assert.success().stdout(concat!("6.8\n",));
}
