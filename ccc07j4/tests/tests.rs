/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("AABBCC\n", "AABCCC\n",)).assert();
    assert.success().stdout(concat!("Is not an anagram.\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("CS AT WATERLOO\n", "COOL AS WET ART\n",))
        .assert();
    assert.success().stdout(concat!("Is an anagram.\n",));
}
