/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n", "0 0\n", "1 0\n", "21 21\n", "20 21\n", "5 3\n", "16 3\n",
        ))
        .assert();
    assert.success().stdout(concat!("3\n"));
}
