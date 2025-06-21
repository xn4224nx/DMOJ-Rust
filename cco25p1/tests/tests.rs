/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("cco25p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6 10\n", "1 1\n", "5 2\n", "200 6\n", "9 2\n", "6 2\n", "100 1\n",
        ))
        .assert();
    assert.success().stdout(concat!("310\n"));
}
