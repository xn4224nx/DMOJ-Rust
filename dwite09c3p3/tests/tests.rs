/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("1\n", "0\n", "11\n", "00\n", "00000000\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("0\n", "8\n", "130\n", "310\n", "1024\n",));
}
