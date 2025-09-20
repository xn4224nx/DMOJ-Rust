/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin("fhc15c2p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n",
            "0.1\n",
            "0.12\n",
            "0.123\n",
            "0.1234\n",
            "0.12345\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "Case #1: 34.64694\n",
        "Case #2: 28.64398\n",
        "Case #3: 27.91171\n",
        "Case #4: 27.81676\n",
        "Case #5: 27.80493\n",
    ));
}
