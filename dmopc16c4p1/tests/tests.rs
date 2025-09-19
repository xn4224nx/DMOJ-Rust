/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dmopc16c4p1").unwrap();
    let assert = cmd.write_stdin("2\n1\n3\n").assert();
    assert.success().stdout("T\nF\n");
}
