/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc08j2").unwrap();
    let assert = cmd.write_stdin("2\n1\n3\n1\n2\n3\n4\n1\n").assert();
    assert.success().stdout("B C D A E\n");
}
