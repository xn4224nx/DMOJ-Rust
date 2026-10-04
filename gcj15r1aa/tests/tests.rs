/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "4\n",
            "10 5 15 5\n",
            "2\n",
            "100 100\n",
            "8\n",
            "81 81 81 81 81 81 81 0\n",
            "6\n",
            "23 90 40 0 100 9\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "Case #1: 15 25\n",
        "Case #2: 0 0\n",
        "Case #3: 81 567\n",
        "Case #4: 181 244\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("1\n", "4\n", "10 5 15 5\n",))
        .assert();
    assert.success().stdout(concat!("Case #1: 15 25\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("1\n", "2\n", "100 100\n",))
        .assert();
    assert.success().stdout(concat!("Case #1: 0 0\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("1\n", "8\n", "81 81 81 81 81 81 81 0\n",))
        .assert();
    assert.success().stdout(concat!("Case #1: 81 567\n",));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("1\n", "6\n", "23 90 40 0 100 9\n",))
        .assert();
    assert.success().stdout(concat!("Case #1: 181 244\n",));
}
