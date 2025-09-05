/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ioi09p5").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3 4\n", "2\n", "3\n", "5\n", "200\n", "100\n", "300\n", "800\n", "3\n", "2\n", "-3\n",
            "1\n", "4\n", "-4\n", "-2\n", "-1\n",
        ))
        .assert();
    assert.success().stdout("5300\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ioi09p5").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2 4\n", "5\n", "2\n", "100\n", "500\n", "1000\n", "2000\n", "3\n", "1\n", "2\n",
            "4\n", "-1\n", "-3\n", "-2\n", "-4\n",
        ))
        .assert();
    assert.success().stdout("16200\n");
}
