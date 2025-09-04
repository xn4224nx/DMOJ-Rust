/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc11j1").unwrap();
    let assert = cmd.write_stdin(concat!("4\n", "5\n",)).assert();
    assert.success().stdout(concat!("VladSaturnian\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc11j1").unwrap();
    let assert = cmd.write_stdin(concat!("2\n", "3\n",)).assert();
    assert
        .success()
        .stdout(concat!("VladSaturnian\n", "GraemeMercurian\n"));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc11j1").unwrap();
    let assert = cmd.write_stdin(concat!("8\n", "6\n",)).assert();
    assert.success().stdout(concat!("",));
}
