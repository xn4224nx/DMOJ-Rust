/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci17c2p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4 5\n", "zagreb\n", "split\n", "zadar\n", "sisak\n", "z\n", "s\n", "s\n", "z\n",
            "z\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "zadar\n", "sisak\n", "split\n", "zagreb\n", "zadar\n",
    ));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci17c2p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5 3\n",
            "london\n",
            "rim\n",
            "pariz\n",
            "moskva\n",
            "sarajevo\n",
            "p\n",
            "r\n",
            "p\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("pariz\n", "rim\n", "pariz\n",));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci17c2p2").unwrap();
    let assert = cmd
        .write_stdin(concat!("1 3\n", "zagreb\n", "z\n", "z\n", "z\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("zagreb\n", "zagreb\n", "zagreb\n",));
}
