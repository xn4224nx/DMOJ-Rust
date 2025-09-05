/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc25j3").unwrap();
    let assert = cmd.write_stdin("1\nAbC3c2Cd9\n").assert();
    assert.success().stdout("ACC14\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc25j3").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n",
            "Ahkiy-6ebvXCV1\n",
            "393hhhUHkbs5gh6QpS-9-8\n",
            "PL12N-2G1234Duytrty8-86tyaYySsDdEe\n"
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("AXCV-5\n", "UHQS387\n", "PLNGDYSDE1166\n",));
}
