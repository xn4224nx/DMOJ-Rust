/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4 3\n", "1 2 2\n", "1 3 5\n", "2 3 2\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("0\n", "2\n", "4\n", "-1\n",));
}
