/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc12j1").unwrap();
    let assert = cmd.write_stdin("40\n39\n").assert();
    assert
        .success()
        .stdout("Congratulations, you are within the speed limit!\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc12j1").unwrap();
    let assert = cmd.write_stdin("100\n131\n").assert();
    assert
        .success()
        .stdout("You are speeding and your fine is $500.\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc12j1").unwrap();
    let assert = cmd.write_stdin("100\n120\n").assert();
    assert
        .success()
        .stdout("You are speeding and your fine is $100.\n");
}
