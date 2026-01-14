/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("57234\n", "00907\n", "34100\n", "99999\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("right 234\n", "right 907\n", "left 100\n",));
}
