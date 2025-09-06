/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc16s2").unwrap();
    let assert = cmd
        .write_stdin(concat!("1\n", "3\n", "5 1 4\n", "6 2 4\n",))
        .assert();
    assert.success().stdout("12\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc16s2").unwrap();
    let assert = cmd
        .write_stdin(concat!("2\n", "3\n", "5 1 4\n", "6 2 4\n",))
        .assert();
    assert.success().stdout("15\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc16s2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2\n",
            "5\n",
            "202 177 189 589 102\n",
            "17 78 1 496 540\n",
        ))
        .assert();
    assert.success().stdout("2016\n");
}
