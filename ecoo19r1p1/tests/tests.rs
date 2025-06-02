/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ecoo19r1p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1 1 10\n", "10\n", "1 3 10\n", "2 9 5\n", "1 1 10\n", "10\n", "1 3 10\n", "2 9 5\n",
            "1 1 10\n", "10\n", "1 3 10\n", "2 9 5\n", "1 1 10\n", "10\n", "1 3 10\n", "2 9 5\n",
            "1 1 10\n", "10\n", "1 3 10\n", "2 9 5\n",
        ))
        .assert();
    assert.success().stdout("9\n3\n9\n3\n9\n3\n9\n3\n9\n3\n");
}
