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
            "5 55\n",
            "georgechen 24\n",
            "kevinwan 42\n",
            "tankibuds 56\n",
            "richardzhang 1\n",
            "tzak 99\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "georgechen will not advance\n",
        "kevinwan will not advance\n",
        "tankibuds will advance\n",
        "richardzhang will not advance\n",
        "tzak will advance\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("1 48\n", "vrotherin 48\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("vrotherin will not advance\n",));
}
