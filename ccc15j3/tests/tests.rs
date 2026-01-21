/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("joy\n",)).assert();
    assert.success().stdout(concat!("jikoyuz\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("ham\n",)).assert();
    assert.success().stdout(concat!("hijamon\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("abcdefghijklmnopqrstuvwzyz\n",))
        .assert();
    assert.success().stdout(concat!(
        "abaccaddefefeggehhiji",
        "jikkillimmonnopopoqqo",
        "rrossuttuvuvuwwuxzuzy",
        "uzzuz\n",
    ));
}
