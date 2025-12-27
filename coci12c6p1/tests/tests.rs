/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin("WA\n").assert();
    assert.success().stdout("13\n");
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin("UNUCIC\n").assert();
    assert.success().stdout("36\n");
}
