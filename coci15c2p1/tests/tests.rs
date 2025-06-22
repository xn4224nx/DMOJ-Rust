/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci15c2p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "tomo\n", "mono\n", "dak\n", "6666\n",))
        .assert();
    assert.success().stdout("1\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci15c2p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("2\n", "ja\n", "la\n", "52\n",))
        .assert();
    assert.success().stdout("2\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci15c2p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "dom\n", "fon\n", "tom\n", "366\n",))
        .assert();
    assert.success().stdout("2\n");
}

#[test]
fn exp_04() {
    let mut cmd = Command::cargo_bin("coci15c2p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("3\n", "mono\n", "dak\n", "tomo\n", "6666\n",))
        .assert();
    assert.success().stdout("1\n");
}

#[test]
fn exp_05() {
    let mut cmd = Command::cargo_bin("coci15c2p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "ygcdnyyb\n",
            "wibfowyb\n",
            "ydhyyurij\n",
            "dfkeegc\n",
            "mmy\n",
            "zhafmyzc\n",
            "94236992\n",
        ))
        .assert();
    assert.success().stdout("3\n");
}
