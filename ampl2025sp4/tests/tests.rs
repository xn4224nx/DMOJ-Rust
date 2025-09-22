/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin("ampl2025sp4").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n", "1 1\n", "1 1\n", "1 2\n", "3 1\n", "3\n", "1 1\n", "4 2\n", "4 3\n",
        ))
        .assert();
    assert.success().stdout("4\n1\n0\n");
}
