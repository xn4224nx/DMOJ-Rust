/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "Ada Alan Grace John\n",
            "John Grace Alan Ada\n",
        ))
        .assert();
    assert.success().stdout(concat!("good\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "7\n",
            "Rich Graeme Michelle Sandy Vlado Ron Jacob\n",
            "Ron Vlado Sandy Michelle Rich Graeme Jacob\n",
        ))
        .assert();
    assert.success().stdout(concat!("bad\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "Ada Alan Grace John\n",
            "Grace John  Alan Ada\n",
        ))
        .assert();
    assert.success().stdout(concat!("bad\n",));
}
