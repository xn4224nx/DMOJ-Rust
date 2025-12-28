/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4 2 2\n", "##\n", "..\n", "..\n", "##\n", "#.\n", ".#\n", ".#\n", "#.\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("0\n", "0\n", "1\n", "1\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1 5 8\n",
            ".....#.#\n",
            "...#..#.\n",
            "..#.#...\n",
            ".#......\n",
            "#.......\n",
        ))
        .assert();
    assert.success().stdout(concat!("4\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2 3 3\n", "...\n", "##.\n", "..#\n", ".#.\n", "#..\n", "..#\n",
        ))
        .assert();
    assert.success().stdout(concat!("1\n", "2\n",));
}
