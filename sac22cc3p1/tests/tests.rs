/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("sac22cc3p1").unwrap();
    let assert = cmd.write_stdin(concat!("200\n", "2\n", "3\n",)).assert();
    assert.success().stdout(concat!("198\n", "196\n", "194\n",));
}
