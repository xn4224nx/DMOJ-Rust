/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci16c4p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n", "ZAGREB\n", "SISAK\n", "ZADAR\n", "ZABOK\n", "ZA\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "****B*D*\n",
        "*G******\n",
        "********\n",
        "********\n",
    ));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci16c4p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "SPLIT\n",
            "VINKOVCI\n",
            "NOVSKA\n",
            "RIJEKA\n",
            "VINKO\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "********\n",
        "********\n",
        "********\n",
        "V*******\n",
    ));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci16c4p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "AAAABCD\n",
            "AAAABCA\n",
            "AAAACDE\n",
            "AAAAAAA\n",
            "AAAA\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "***ABC**\n",
        "********\n",
        "********\n",
        "********\n",
    ));
}
