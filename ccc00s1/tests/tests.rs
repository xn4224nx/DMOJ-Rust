/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc00s1").unwrap();
    let assert = cmd.write_stdin("48\n3\n10\n4\n").assert();
    assert
        .success()
        .stdout("Martha plays 66 times before going broke.\n");
}
