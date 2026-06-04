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
            "5\n", "6\n", "1###..\n", "##4#5.\n", "2#####\n", ".#.#..\n", "...#.3\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("1\n", "6\n", "4\n", "1\n", "4\n",));
}
