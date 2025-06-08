/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci19c5p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6 7\n",
            "***....\n",
            "***..**\n",
            ".....**\n",
            ".***.**\n",
            ".***...\n",
            ".***...\n",
        ))
        .assert();
    assert.success().stdout("3\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci19c5p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("3 3\n", "*.*\n", "...\n", "*.*\n",))
        .assert();
    assert.success().stdout("4\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci19c5p1").unwrap();
    let assert = cmd.write_stdin(concat!("1 10\n", ".*.**.***.\n",)).assert();
    assert.success().stdout("3\n");
}
