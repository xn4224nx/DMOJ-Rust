/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("crci06p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "3\n", "2 1 2\n", "3 2 3 4\n", "3 4 2 1\n",))
        .assert();
    assert.success().stdout(concat!("1\n", "2\n", "4\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("crci06p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "8\n",
            "5\n",
            "4 1 3 5 4\n",
            "2 5 6\n",
            "3 6 7 8\n",
            "2 6 2\n",
            "4 2 6 8 1\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("1\n", "2\n", "6\n", "8\n",));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("crci06p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("5\n", "3\n", "2 1 3\n", "2 2 1\n", "4 2 1 4 5\n",))
        .assert();
    assert.success().stdout(concat!("1\n",));
}
