/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ioi04p5").unwrap();
    let assert = cmd
        .write_stdin(concat!("17 3 3\n", "13 4 8\n", "4 8 6\n",))
        .assert();
    assert.success().stdout("17\n");
}
