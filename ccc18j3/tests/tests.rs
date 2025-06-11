/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc18j3").unwrap();
    let assert = cmd.write_stdin("3 10 12 5\n").assert();
    assert.success().stdout(concat!(
        "0 3 13 25 30\n",
        "3 0 10 22 27\n",
        "13 10 0 12 17\n",
        "25 22 12 0 5\n",
        "30 27 17 5 0\n",
    ));
}
