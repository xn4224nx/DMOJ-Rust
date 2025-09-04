/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc17j2").unwrap();
    let assert = cmd.write_stdin("12\n3\n").assert();
    assert.success().stdout("13332\n");
}
