/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("acmtryouts0b").unwrap();
    let assert = cmd.write_stdin("2\n1\nA\nB\n3\nZEO\nNXF\n").assert();
    assert.success().stdout("BA\nFOXENZ\n");
}
