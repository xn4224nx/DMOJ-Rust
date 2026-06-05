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
            "5 6\n", "--t---\n", "--stst\n", "tttsss\n", "----s-\n", "--tsss\n",
        ))
        .assert();
    assert.success().stdout(concat!("7\n",));
}
