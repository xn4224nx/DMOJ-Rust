/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("AFB+8HC-4\n",)).assert();
    assert
        .success()
        .stdout(concat!("AFB tighten 8\n", "HC loosen 4\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("AFB+8SC-4H-2GDPE+9\n",)).assert();
    assert.success().stdout(concat!(
        "AFB tighten 8\n",
        "SC loosen 4\n",
        "H loosen 2\n",
        "GDPE tighten 9\n",
    ));
}
