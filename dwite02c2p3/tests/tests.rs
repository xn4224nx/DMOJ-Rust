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
            "10 10\n",
            "..XX..XXXX\n",
            ".X.XX.XX..\n",
            "XX....X..X\n",
            "X...XX..XX\n",
            "X..X.X....\n",
            "..........\n",
            ".XXX.....X\n",
            ".......XXX\n",
            "......XX..\n",
            "X.X.X.X...\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("38\n", "30\n", "27\n", "0\n", "0\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "8 14\n",
            "..............\n",
            ".XX...........\n",
            ".XX...X....X..\n",
            "......X...X.X.\n",
            "......X....X..\n",
            "..............\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("11\n", "11\n", "11\n", "11\n", "11\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4 12\n",
            "XX..........\n",
            "XX...X....X.\n",
            ".....X...X.X\n",
            ".....X....X.\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("11\n", "11\n", "11\n", "11\n", "11\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3 3\n", ".X.\n", ".X.\n", ".X.\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("3\n", "3\n", "3\n", "3\n", "3\n",));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1 1\n", ".\n",)).assert();
    assert
        .success()
        .stdout(concat!("0\n", "0\n", "0\n", "0\n", "0\n",));
}
