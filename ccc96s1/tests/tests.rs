/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "4\n", "6\n", "12\n",))
        .assert();
    assert.success().stdout(concat!(
        "4 is a deficient number.\n",
        "6 is a perfect number.\n",
        "12 is an abundant number.\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "28\n",)).assert();
    assert
        .success()
        .stdout(concat!("28 is a perfect number.\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "496\n",)).assert();
    assert
        .success()
        .stdout(concat!("496 is a perfect number.\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "8128\n",)).assert();
    assert
        .success()
        .stdout(concat!("8128 is a perfect number.\n",));
}
