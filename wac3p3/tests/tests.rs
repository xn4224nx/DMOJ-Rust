/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("wac3p3").unwrap();
    let assert = cmd
        .write_stdin(concat!("RLUDD\n", "1\n", "RLU 4\n",))
        .assert();
    assert.success().stdout("9\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("wac3p3").unwrap();
    let assert = cmd
        .write_stdin(concat!("UDULRL\n", "2\n", "ULR 3\n", "UDU 2\n",))
        .assert();
    assert.success().stdout("8\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("wac3p3").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "ULRURURU\n",
            "3\n",
            "LRURU 500\n",
            "LRUR 750\n",
            "URU 1000\n",
        ))
        .assert();
    assert.success().stdout("508\n");
}
