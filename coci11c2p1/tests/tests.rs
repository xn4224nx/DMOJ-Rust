/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "20\n", "30\n", "50\n", "48\n", "33\n", "66\n", "0\n", "64\n",
        ))
        .assert();
    assert.success().stdout(concat!("261\n", "3 4 5 6 8\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "20\n", "0\n", "50\n", "80\n", "77\n", "110\n", "56\n", "48\n",
        ))
        .assert();
    assert.success().stdout(concat!("373\n", "3 4 5 6 7\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "20\n", "30\n", "50\n", "80\n", "110\n", "11\n", "0\n", "85\n",
        ))
        .assert();
    assert.success().stdout(concat!("355\n", "2 3 4 5 8\n",));
}
