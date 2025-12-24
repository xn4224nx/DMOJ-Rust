/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("3 100 10\n", "20 59 4\n", "30 69 4\n", "40 79 4\n",))
        .assert();
    assert.success().stdout("80\n");
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("2 10 4\n", "3 5 2\n", "5 7 2\n",))
        .assert();
    assert.success().stdout("9\n");
}
