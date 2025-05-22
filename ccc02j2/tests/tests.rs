/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc02j2").unwrap();
    let assert = cmd.write_stdin("color\nfor\ntaylor\nquit!\n").assert();
    assert.success().stdout("colour\nfor\ntaylour\n");
}
