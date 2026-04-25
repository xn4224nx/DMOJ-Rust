/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "MXACY\n", "CZBNP\n", "PQRST\n", "PQRMS\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("NEGATIVE MARKS\n", "FAIL\n", "PASS\n", "FAIL\n",));
}
