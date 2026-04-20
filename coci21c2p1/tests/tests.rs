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
            "3\n",
            "section zivotinje\n",
            "section boje\n",
            "section voce\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("1 zivotinje\n", "2 boje\n", "3 voce\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "section zivotinje\n",
            "subsection macke\n",
            "subsection psi\n",
            "subsubsection mops\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "1 zivotinje\n",
        "1.1 macke\n",
        "1.2 psi\n",
        "1.2.1 mops\n",
    ));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "section zivotinje\n",
            "subsection psi\n",
            "section voce\n",
            "subsection ananas\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "1 zivotinje\n",
        "1.1 psi\n",
        "2 voce\n",
        "2.1 ananas\n",
    ));
}
