/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dmopc20c3p1").unwrap();
    let assert = cmd.write_stdin("4\n4 2 3 1\n").assert();
    assert.success().stdout("YES\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("dmopc20c3p1").unwrap();
    let assert = cmd.write_stdin("10\n1 3 2 4 5 8 6 3 7 10\n").assert();
    assert.success().stdout("NO\n");
}
