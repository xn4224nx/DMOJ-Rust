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
            "i\n", "20\n", "10\n", "i\n", "20\n", "9\n", "n\n", "20\n", "f\n", "20\n", "s\n",
            "20\n", "6\n", "q\n",
        ))
        .assert();
    assert.success().stdout(concat!("2\n", "3\n", "4\n",));
}
