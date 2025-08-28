/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("sac22cc5p2").unwrap();
    let assert = cmd.write_stdin(concat!("2\n", "left\n",)).assert();
    assert.success().stdout(concat!("right\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("sac22cc5p2").unwrap();
    let assert = cmd.write_stdin(concat!("15\n", "right\n",)).assert();
    assert.success().stdout(concat!("right\n",));
}
