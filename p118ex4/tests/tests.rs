/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("5\n",)).assert();
    assert.success().stdout(concat!(
        "5 X 1 = 5\n",
        "5 X 2 = 10\n",
        "5 X 3 = 15\n",
        "5 X 4 = 20\n",
        "5 X 5 = 25\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("8\n",)).assert();
    assert.success().stdout(concat!(
        "8 X 1 = 8\n",
        "8 X 2 = 16\n",
        "8 X 3 = 24\n",
        "8 X 4 = 32\n",
        "8 X 5 = 40\n",
        "8 X 6 = 48\n",
        "8 X 7 = 56\n",
        "8 X 8 = 64\n",
    ));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n",)).assert();
    assert.success().stdout(concat!("1 X 1 = 1\n",));
}
