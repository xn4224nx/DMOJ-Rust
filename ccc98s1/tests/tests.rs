/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2\n",
            "The quick brown fox jumps over the lazy dog\n",
            "Now is the time for all good people to come to the aid of the party\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "The quick brown fox jumps **** the **** dog\n",
        "Now is the **** for all **** people to **** to the aid of the party\n",
    ));
}
