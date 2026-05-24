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
            "4\n", "7\n", "2 3\n", "1 4\n", "1 2\n", "1 2\n", "2 2\n", "2 2\n", "2 1\n",
        ))
        .assert();
    assert.success().stdout(concat!("6\n",));
}
