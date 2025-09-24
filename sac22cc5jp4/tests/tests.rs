/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin("sac22cc5jp4").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n",
            "0\n",
            "0\n",
            "3 1 2 5\n",
            "3 1 2 3\n",
            "1 2\n",
        ))
        .assert();
    assert.success().stdout("1 2 5 3 4\n");
}
