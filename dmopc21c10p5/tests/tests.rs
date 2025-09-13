/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dmopc21c10p5").unwrap();
    let assert = cmd.write_stdin("").assert();
    assert.success().stdout("15230439315\n");
}
