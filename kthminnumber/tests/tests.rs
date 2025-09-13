/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("kthminnumber").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5 5\n",
            "1 3 4 5 1\n",
            "1 5 3\n",
            "0 6 0\n",
            "6 6 4\n",
            "5 6 6\n",
            "2 6 6\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("3\n", "5\n", "4\n", "3\n", "5\n",));
}
