/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n",
            "4\n",
            "3 10 8 14\n",
            "1 11 12 12\n",
            "6 2 3 9\n",
        ))
        .assert();
    assert.success().stdout(concat!("yes\n",));
}
