/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin("coci07c2p3").unwrap();
    let assert = cmd
        .write_stdin(concat!("4 4\n", "luka\n", "o#a#\n", "kula\n", "i#a#\n",))
        .assert();
    assert.success().stdout("kala\n");
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin("coci07c2p3").unwrap();
    let assert = cmd
        .write_stdin(concat!("4 4\n", "luka\n", "o#a#\n", "kula\n", "i#as\n",))
        .assert();
    assert.success().stdout("as\n");
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin("coci07c2p3").unwrap();
    let assert = cmd
        .write_stdin(concat!("4 5\n", "adaca\n", "da##b\n", "abb#b\n", "abbac\n",))
        .assert();
    assert.success().stdout("abb\n");
}
