/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2\n",
            "3 3\n",
            "30 10 40\n",
            "20 50 60\n",
            "60 60 50\n",
            "5 2\n",
            "1 1000000000\n",
            "500000000 1000000000\n",
            "1 1000000000\n",
            "500000000 1\n",
            "1 1000000000\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("Case #1: 110\n", "Case #2: 4999999996\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1\n",
            "3 3\n",
            "30 10 40\n",
            "20 50 60\n",
            "60 60 50\n",
        ))
        .assert();
    assert.success().stdout(concat!("Case #1: 110\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1\n",
            "5 2\n",
            "1 1000000000\n",
            "500000000 1000000000\n",
            "1 1000000000\n",
            "500000000 1\n",
            "1 1000000000\n",
        ))
        .assert();
    assert.success().stdout(concat!("Case #1: 4999999996\n",));
}
