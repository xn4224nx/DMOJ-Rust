/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc13j2").unwrap();
    let assert = cmd.write_stdin("SHINS\n").assert();
    assert.success().stdout("YES\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc13j2").unwrap();
    let assert = cmd.write_stdin("NOISE\n").assert();
    assert.success().stdout("NO\n");
}
