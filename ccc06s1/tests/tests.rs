/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "AABbCcddEe\n",
            "AaBBccddee\n",
            "5\n",
            "ABCdE\n",
            "aBcdE\n",
            "ABcdE\n",
            "ABCde\n",
            "ABcDe\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "Possible baby.\n",
        "Not their baby!\n",
        "Possible baby.\n",
        "Possible baby.\n",
        "Not their baby!\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("AABbCcddEe\n", "AaBBccddee\n", "1\n", "ABCdE\n",))
        .assert();
    assert.success().stdout(concat!("Possible baby.\n",));
}
#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("AABbCcddEe\n", "AaBBccddee\n", "1\n", "aBcdE\n",))
        .assert();
    assert.success().stdout(concat!("Not their baby!\n",));
}
#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("AABbCcddEe\n", "AaBBccddee\n", "1\n", "ABcdE\n",))
        .assert();
    assert.success().stdout(concat!("Possible baby.\n",));
}
#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("AABbCcddEe\n", "AaBBccddee\n", "1\n", "ABCde\n",))
        .assert();
    assert.success().stdout(concat!("Possible baby.\n",));
}
#[test]
fn full_program_exp5() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("AABbCcddEe\n", "AaBBccddee\n", "1\n", "ABcDe\n",))
        .assert();
    assert.success().stdout(concat!("Not their baby!\n",));
}
