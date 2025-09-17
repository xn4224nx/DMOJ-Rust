/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("geometry1").unwrap();
    let assert = cmd.write_stdin("1\n0 0 0 1 1 1\n").assert();
    assert.success().stdout("0.50 3.41\n");
}
