/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "70\n", "62\n", "58\n", "73\n",))
        .assert();
    assert.success().stdout(concat!("62 1\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "8\n", "75\n", "70\n", "60\n", "70\n", "70\n", "60\n", "75\n", "70\n",
        ))
        .assert();
    assert.success().stdout(concat!("60 2\n",));
}
