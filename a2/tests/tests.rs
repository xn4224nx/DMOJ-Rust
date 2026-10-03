/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("Fr\n", "qp\n", "HH\n", "db\n", "  \n", "pq\n",))
        .assert();
    assert.success().stdout(concat!(
        "Ready\n",
        "Ordinary pair\n",
        "Mirrored pair\n",
        "Ordinary pair\n",
        "Mirrored pair\n",
    ));
}
