/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n",
            "88-SNOW-5555\n",
            "519-888-4567\n",
            "BUY-MORE-POP\n",
            "416-PIZZA-BOX\n",
            "5059381123\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "887-669-5555\n",
        "519-888-4567\n",
        "289-667-3767\n",
        "416-749-9226\n",
        "505-938-1123\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "88-SNOW-5555\n",)).assert();
    assert.success().stdout(concat!("887-669-5555\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "519-888-4567\n",)).assert();
    assert.success().stdout(concat!("519-888-4567\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "BUY-MORE-POP\n",)).assert();
    assert.success().stdout(concat!("289-667-3767\n",));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "416-PIZZA-BOX\n",)).assert();
    assert.success().stdout(concat!("416-749-9226\n",));
}

#[test]
fn full_program_exp5() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "5059381123\n",)).assert();
    assert.success().stdout(concat!("505-938-1123\n",));
}

#[test]
fn full_program_exp6() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "PSTVWZ1111\n",)).assert();
    assert.success().stdout(concat!("778-899-1111\n",));
}
