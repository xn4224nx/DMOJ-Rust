/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("cco96p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "6\n",
            "9793 0/1\n",
            "2384 0/2\n",
            "6264 0/1\n",
            "3383 1/0\n",
            "2795 0/0\n",
            "0218 1/0\n",
            "1\n",
            "1234 4/0\n",
            "1\n",
            "1234 2/2\n",
            "2\n",
            "6428 3/0\n",
            "1357 3/0\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "3411\n",
        "1234\n",
        "indeterminate\n",
        "impossible\n",
    ));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("cco96p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1\n",
            "6\n",
            "9793 0/1\n",
            "2384 0/2\n",
            "6264 0/1\n",
            "3383 1/0\n",
            "2795 0/0\n",
            "0218 1/0\n",
        ))
        .assert();
    assert.success().stdout(concat!("3411\n",));
}
