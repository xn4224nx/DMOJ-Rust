/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n",
            "2\n",
            "1 2 3 4\n",
            "5 6 7 8\n",
            "9 10 11 12\n",
            "13 14 15 16\n",
            "3\n",
            "1 2 5 4\n",
            "3 11 6 15\n",
            "9 10 7 12\n",
            "13 14 8 16\n",
            "2\n",
            "1 2 3 4\n",
            "5 6 7 8\n",
            "9 10 11 12\n",
            "13 14 15 16\n",
            "2\n",
            "1 2 3 4\n",
            "5 6 7 8\n",
            "9 10 11 12\n",
            "13 14 15 16\n",
            "2\n",
            "1 2 3 4\n",
            "5 6 7 8\n",
            "9 10 11 12\n",
            "13 14 15 16\n",
            "3\n",
            "1 2 3 4\n",
            "5 6 7 8\n",
            "9 10 11 12\n",
            "13 14 15 16\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "Case #1: 7\n",
        "Case #2: Bad magician!\n",
        "Case #3: Volunteer cheated!\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1\n",
            "2\n",
            "1 2 3 4\n",
            "5 6 7 8\n",
            "9 10 11 12\n",
            "13 14 15 16\n",
            "3\n",
            "1 2 5 4\n",
            "3 11 6 15\n",
            "9 10 7 12\n",
            "13 14 8 16\n",
        ))
        .assert();
    assert.success().stdout(concat!("Case #1: 7\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1\n",
            "2\n",
            "1 2 3 4\n",
            "5 6 7 8\n",
            "9 10 11 12\n",
            "13 14 15 16\n",
            "2\n",
            "1 2 3 4\n",
            "5 6 7 8\n",
            "9 10 11 12\n",
            "13 14 15 16\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("Case #1: Bad magician!\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1\n",
            "2\n",
            "1 2 3 4\n",
            "5 6 7 8\n",
            "9 10 11 12\n",
            "13 14 15 16\n",
            "3\n",
            "1 2 3 4\n",
            "5 6 7 8\n",
            "9 10 11 12\n",
            "13 14 15 16\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("Case #1: Volunteer cheated!\n",));
}
