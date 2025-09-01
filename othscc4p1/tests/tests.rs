/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("othscc4p1").unwrap();
    let assert = cmd.write_stdin(concat!("8\n",)).assert();
    assert.success().stdout(concat!("64\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("othscc4p1").unwrap();
    let assert = cmd.write_stdin(concat!("4\n",)).assert();
    assert.success().stdout(concat!("32\n",));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("othscc4p1").unwrap();
    let assert = cmd.write_stdin(concat!("17279317\n",)).assert();
    assert.success().stdout(concat!("138234536\n",));
}
