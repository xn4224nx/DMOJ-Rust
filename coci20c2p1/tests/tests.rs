/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci20c2p1").unwrap();
    let assert = cmd.write_stdin(concat!("7\n", "++---==\n",)).assert();
    assert
        .success()
        .stdout(concat!("./\\....\n", "/..\\...\n", "....\\__\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci20c2p1").unwrap();
    let assert = cmd.write_stdin(concat!("5\n", "+=+=+\n",)).assert();
    assert
        .success()
        .stdout(concat!("..._/\n", "._/..\n", "/....\n",));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci20c2p1").unwrap();
    let assert = cmd.write_stdin(concat!("4\n", "--=+\n",)).assert();
    assert.success().stdout(concat!("\\...\n", ".\\_/\n",));
}
