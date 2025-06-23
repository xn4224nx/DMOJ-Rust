/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci14c2p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n", "leo\n", "kiki\n", "eden\n", "eden\n", "kiki\n",
        ))
        .assert();
    assert.success().stdout("leo\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci14c2p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n", "marina\n", "josipa\n", "nikola\n", "vinko\n", "filipa\n", "josipa\n",
            "filipa\n", "marina\n", "nikola\n",
        ))
        .assert();
    assert.success().stdout("vinko\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci14c2p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n", "mislav\n", "stanko\n", "mislav\n", "ana\n", "stanko\n", "ana\n", "mislav\n",
        ))
        .assert();
    assert.success().stdout("mislav\n");
}
