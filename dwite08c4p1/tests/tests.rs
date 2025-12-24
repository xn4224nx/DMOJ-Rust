/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n", "0\n", "0\n", "-5\n", "5\n", "5\n", "0\n", "-5\n", "5\n", "5\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "100,25\n", "50,0\n", "75,50\n", "50,0\n", "75,50\n",
    ));
}
