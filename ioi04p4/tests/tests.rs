/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ioi04p4").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "21 11\n", "4\n", "10 4\n", "6 2\n", "7 5\n", "15 10\n",
        ))
        .assert();
    assert.success().stdout("10\n");
}
