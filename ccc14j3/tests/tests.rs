/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc14j3").unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "5 6\n", "6 6\n", "4 3\n", "5 2\n",))
        .assert();
    assert.success().stdout("94\n91\n");
}
