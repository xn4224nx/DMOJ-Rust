/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "2\n", "5\n", "3\n", "12\n",))
        .assert();
    assert.success().stdout(concat!("Byron\n",));
}
