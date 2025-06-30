/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("crci06p3").unwrap();
    let assert = cmd
        .write_stdin(concat!("6 7\n", "1\n", "5\n", "3\n", "3\n", "5\n", "1\n",))
        .assert();
    assert.success().stdout("2 3\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("crci06p3").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "14 5\n", "1\n", "3\n", "4\n", "2\n", "2\n", "4\n", "3\n", "4\n", "3\n", "3\n", "3\n",
            "2\n", "3\n", "3\n",
        ))
        .assert();
    assert.success().stdout("7 2\n");
}
