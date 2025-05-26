/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("sixteenbitswonly").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n",
            "1 1 2\n",
            "2147483647 2147483647 4611686014132420610\n",
            "12345678 87654321 1082152022374638\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "16 BIT S/W ONLY\n",
        "16 BIT S/W ONLY\n",
        "POSSIBLE DOUBLE SIGMA\n",
    ));
}
