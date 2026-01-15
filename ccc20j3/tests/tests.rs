/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n", "44,62\n", "34,69\n", "24,78\n", "42,44\n", "64,10\n",
        ))
        .assert();
    assert.success().stdout(concat!("23,9\n", "65,79\n",));
}
