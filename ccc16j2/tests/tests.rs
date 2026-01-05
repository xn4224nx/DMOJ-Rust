/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "16 3 2 13\n",
            "5 10 11 8\n",
            "9 6 7 12\n",
            "4 15 14 1\n",
        ))
        .assert();
    assert.success().stdout(concat!("magic\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5 10 1 3\n",
            "10 4 2 3\n",
            "1 2 8 5\n",
            "3 3 5 0\n",
        ))
        .assert();
    assert.success().stdout(concat!("not magic\n",));
}
