/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("phantom1").unwrap();
    let assert = cmd
        .write_stdin(concat!("2\n", "1 10\n", "1 100\n",))
        .assert();
    assert.success().stdout(concat!("4\n", "25\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("phantom1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "1 20\n",
            "70 100\n",
            "800 1000\n",
            "800 997\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("8\n", "6\n", "29\n", "28\n",));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("phantom1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "45 5000\n",
            "45 4999\n",
            "502 5000\n",
            "2 8932\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("655\n", "654\n", "574\n", "1110\n",));
}
