/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci20c1p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6 6\n", "..>>>v\n", ".o^..v\n", ".v.<.v\n", ".>>^.v\n", ".x<<<<\n", "......\n",
        ))
        .assert();
    assert.success().stdout(concat!(":)\n", "E\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci20c1p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5 5\n", "v<<<<\n", ">v.>^\n", "v<.o.\n", ">>v>v\n", "..>>x\n",
        ))
        .assert();
    assert.success().stdout(concat!(":)\n", "S\n",));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci20c1p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("3 3\n", "x>.\n", ".o^\n", "^<.\n",))
        .assert();
    assert.success().stdout(concat!(":(\n",));
}
