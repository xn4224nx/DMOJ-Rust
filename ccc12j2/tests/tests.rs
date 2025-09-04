/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc12j2").unwrap();
    let assert = cmd.write_stdin("30\n10\n20\n20\n").assert();
    assert.success().stdout("No Fish\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc12j2").unwrap();
    let assert = cmd.write_stdin("3\n4\n7\n9\n").assert();
    assert.success().stdout("Fish Rising\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc12j2").unwrap();
    let assert = cmd.write_stdin("9\n6\n5\n2\n").assert();
    assert.success().stdout("Fish Diving\n");
}

#[test]
fn exp_04() {
    let mut cmd = Command::cargo_bin("ccc12j2").unwrap();
    let assert = cmd.write_stdin("1\n1\n1\n1\n").assert();
    assert.success().stdout("Fish At Constant Depth\n");
}
