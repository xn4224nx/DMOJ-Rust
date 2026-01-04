/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "9 +\n", "3 -\n", "12 A\n", "2 X\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("+++++++++\n", "---\n", "AAAAAAAAAAAA\n", "XX\n",));
}
