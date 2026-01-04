/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n", "2\n", "Easy\n", "Smart\n", "Soft\n", "pie\n", "rock\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "Easy as pie\n",
        "Easy as rock\n",
        "Smart as pie\n",
        "Smart as rock\n",
        "Soft as pie\n",
        "Soft as rock\n",
    ));
}
