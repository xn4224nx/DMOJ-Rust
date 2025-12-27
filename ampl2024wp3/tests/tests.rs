/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1 2 3 4 5 6 7 8 9 10\n",
            "2 4 6 8 10 12 14 16 18 20\n"
        ))
        .assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "31 41 59 26 53 58 98 93 23 84\n",
            "27 18 28 18 28 45 90 45 23 53\n"
        ))
        .assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1 2 4 8 16 32 64 128 256 512\n",
            "1 2 4 8 16 32 64 128 256 512\n"
        ))
        .assert();
    assert.success().stdout(concat!("31\n",));
}
