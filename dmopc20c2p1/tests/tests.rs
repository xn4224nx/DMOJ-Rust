/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dmopc20c2p1").unwrap();
    let assert = cmd.write_stdin(concat!("6\n", "vv>^^^\n",)).assert();
    assert
        .success()
        .stdout(concat!("...../\n", "\\.../.\n", ".\\_/..\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("dmopc20c2p1").unwrap();
    let assert = cmd.write_stdin(concat!("5\n", "vvvvv\n",)).assert();
    assert.success().stdout(concat!(
        "\\....\n", ".\\...\n", "..\\..\n", "...\\.\n", "....\\\n",
    ));
}
