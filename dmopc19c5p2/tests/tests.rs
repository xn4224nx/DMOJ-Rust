/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dmopc19c5p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3 100\n", "A 50\n", "D 10\n", "A 100\n", "A 90\n", "D 0\n", "A 0\n",
        ))
        .assert();
    assert.success().stdout(concat!("DEFEAT\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("dmopc19c5p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4 100\n", "D 10\n", "D 20\n", "D 30\n", "D 30\n", "D 10\n", "D 20\n", "D 30\n",
            "D 40\n",
        ))
        .assert();
    assert.success().stdout(concat!("VICTORY\n",));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("dmopc19c5p2").unwrap();
    let assert = cmd
        .write_stdin(concat!("2 100\n", "A 99\n", "A 99\n", "D 1\n", "A 1\n",))
        .assert();
    assert.success().stdout(concat!("TIE\n",));
}

#[test]
fn exp_04() {
    let mut cmd = Command::cargo_bin("dmopc19c5p2").unwrap();
    let assert = cmd
        .write_stdin(concat!("2 100\n", "A 90\n", "D 10\n", "A 90\n", "D 10\n",))
        .assert();
    assert.success().stdout(concat!("DEFEAT\n",));
}
