/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ecoo13r1p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "23\nTAKE\nTAKE\nSERVE\nTAKE\nSERVE\n",
            "SERVE\nCLOSE\nTAKE\nTAKE\nTAKE\nSERVE\n",
            "CLOSE\nTAKE\nSERVE\nTAKE\nSERVE\nTAKE\n",
            "TAKE\nTAKE\nTAKE\nTAKE\nTAKE\nSERVE\n",
            "CLOSE\nEOF\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("3 0 26\n", "3 2 29\n", "8 5 37\n",));
}
