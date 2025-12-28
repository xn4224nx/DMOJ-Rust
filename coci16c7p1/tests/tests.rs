/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4 3\n",
            "1 5 2\n",
            "2 3 4\n",
            "4 3 2\n",
            "5 4 6\n",
            "3\n",
            "-1 -1 2\n",
            "-1 3 2\n",
            "-1 -1 -1\n",
        ))
        .assert();
    assert.success().stdout(concat!("2\n", "1\n", "4\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3 8\n",
            "6 5 97 99 82 50 95 1\n",
            "85 62 11 64 94 84 88 19\n",
            "43 99 11 64 94 84 31 19\n",
            "3\n",
            "-1 -1 11 64 94 84 -1 19\n",
            "-1 -1 -1 99 -1 -1 -1 1\n",
            "95 -1 -1 -1 -1 80 -1 -1\n",
        ))
        .assert();
    assert.success().stdout(concat!("2\n", "1\n", "0\n",));
}
