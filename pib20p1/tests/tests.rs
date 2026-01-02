/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("5\n", "-1 -5 3 2 0\n",)).assert();
    assert.success().stdout(concat!("2\n",));
}
