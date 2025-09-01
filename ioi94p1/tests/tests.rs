/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ioi94p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n",
            "7\n",
            "3 8\n",
            "8 1 0\n",
            "2 7 4 4\n",
            "4 5 2 6 5\n",
        ))
        .assert();
    assert.success().stdout(concat!("30\n",));
}
