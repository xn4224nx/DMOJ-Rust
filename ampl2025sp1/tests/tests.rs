/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin("ampl2025sp1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "AMPLITUDEISALLINONAI\n",
            "AMPLITUDE\n",
            "AI\n",
            "IA\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("YES\n", "NO\n", "YES\n", "NO\n",));
}
