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
            "5\n", "1\n", "2\n", "3\n", "4\n", "5\n", "3\n", "0 4\n", "1 3\n", "2 2\n",
        ))
        .assert();
    assert.success().stdout(concat!("15\n", "9\n", "3\n",));
}
