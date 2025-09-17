/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci12c3p3").unwrap();
    let assert = cmd
        .write_stdin(concat!("4 2\n", "IVA\n", "IVO\n", "ANA\n", "TOM\n",))
        .assert();
    assert.success().stdout("5\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci12c3p3").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6 3\n",
            "CYNTHIA\n",
            "LLOYD\n",
            "STEVIE\n",
            "KEVIN\n",
            "MALCOLM\n",
            "DABNEY\n",
        ))
        .assert();
    assert.success().stdout("2\n");
}
