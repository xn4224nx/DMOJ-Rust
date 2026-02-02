/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "100\n", "6\n", "50\n", "30\n", "10\n", "10\n", "40\n", "50\n",
        ))
        .assert();
    assert.success().stdout(concat!("5\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("100\n", "3\n", "150\n", "1\n", "1\n",))
        .assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("100\n", "3\n", "1\n", "1\n", "1\n",))
        .assert();
    assert.success().stdout(concat!("3\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "100\n", "8\n", "1\n", "1\n", "1\n", "1\n", "1\n", "1\n", "1\n", "1\n",
        ))
        .assert();
    assert.success().stdout(concat!("8\n",));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("100\n", "1\n", "1\n",)).assert();
    assert.success().stdout(concat!("1\n",));
}

#[test]
fn full_program_exp5() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("100\n", "2\n", "1\n", "101\n",))
        .assert();
    assert.success().stdout(concat!("1\n",));
}
