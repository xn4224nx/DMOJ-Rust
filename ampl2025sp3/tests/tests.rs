/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin("ampl2025sp3").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n", "1 1\n", "2 4 2\n", "1 3\n", "2 1 2\n", "2 3 4\n", "0\n",
        ))
        .assert();
    assert.success().stdout("2\n");
}
