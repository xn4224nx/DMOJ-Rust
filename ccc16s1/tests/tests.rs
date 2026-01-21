/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("abba\n", "baaa\n",)).assert();
    assert.success().stdout(concat!("N\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("cccrocks\n", "socc*rk*\n",))
        .assert();
    assert.success().stdout(concat!("A\n",));
}
