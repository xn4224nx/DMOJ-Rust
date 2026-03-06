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
            "AC\n", "AD\n", "AE\n", "CE\n", "CF\n", "ED\n", "GF\n", "BG\n", "HB\n", "GH\n", "**\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "CF\n",
        "FG\n",
        "There are 2 disconnecting roads.\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("CA\n", "CD\n", "ED\n", "EB\n", "**\n",))
        .assert();
    assert.success().stdout(concat!(
        "AC\n",
        "CD\n",
        "DE\n",
        "EB\n",
        "There are 4 disconnecting roads.\n",
    ));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("AC\n", "AD\n", "DB\n", "CB\n", "**\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("There are 0 disconnecting roads.\n",));
}
