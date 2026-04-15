/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp00() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "BELLS\n",
            "TELLS\n",
            "FALLS\n",
            "DOLLS\n",
            "DULLS\n",
            "DOLLIES\n",
            "SELLS\n",
            "*ELLS\n",
            "D?LLS\n",
            "D?LL*\n",
            "*LL*\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "NO MATCH\n",
        "BELLS, TELLS\n",
        "DOLLS, DULLS\n",
        "DOLLS, DULLS, DOLLIES\n",
        "BELLS, TELLS, FALLS, DOLLS, DULLS, DOLLIES\n",
    ));
}

#[test]
fn full_program_exp01() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n",
            "PRIME.PAS\n",
            "PRINTER.PAS\n",
            "PRINTER.PASR\n",
            "*.PAS\n",
            "*.PAS\n",
            "*.PAS\n",
            "*.PAS\n",
            "*.PAS\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "PRIME.PAS, PRINTER.PAS\n",
        "PRIME.PAS, PRINTER.PAS\n",
        "PRIME.PAS, PRINTER.PAS\n",
        "PRIME.PAS, PRINTER.PAS\n",
        "PRIME.PAS, PRINTER.PAS\n",
    ));
}

#[test]
fn full_program_exp02() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "TSTING.PAS\n",
            "PRIME.TST\n",
            "ATSTART.EXE\n",
            "TST\n",
            "TS\n",
            "TSST\n",
            "*TST*\n",
            "*TST*\n",
            "*TST*\n",
            "*TST*\n",
            "*TST*\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "TSTING.PAS, PRIME.TST, ATSTART.EXE, TST\n",
        "TSTING.PAS, PRIME.TST, ATSTART.EXE, TST\n",
        "TSTING.PAS, PRIME.TST, ATSTART.EXE, TST\n",
        "TSTING.PAS, PRIME.TST, ATSTART.EXE, TST\n",
        "TSTING.PAS, PRIME.TST, ATSTART.EXE, TST\n",
    ));
}

#[test]
fn full_program_exp03() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "PSY\n",
            "PSYCHIC\n",
            "PSYCHED\n",
            "PSYCHOS\n",
            "FGPSY\n",
            "HPSYCHE\n",
            "PSY????\n",
            "PSY????\n",
            "PSY????\n",
            "PSY????\n",
            "PSY????\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "PSYCHIC, PSYCHED, PSYCHOS\n",
        "PSYCHIC, PSYCHED, PSYCHOS\n",
        "PSYCHIC, PSYCHED, PSYCHOS\n",
        "PSYCHIC, PSYCHED, PSYCHOS\n",
        "PSYCHIC, PSYCHED, PSYCHOS\n",
    ));
}

#[test]
fn full_program_exp04() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "BELLS\n",
            "TELLS\n",
            "FALLS\n",
            "DOLLS\n",
            "DULLS\n",
            "DOLLIES\n",
            "SELLS\n",
            "SELLS\n",
            "SELLS\n",
            "SELLS\n",
            "SELLS\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "NO MATCH\n",
        "NO MATCH\n",
        "NO MATCH\n",
        "NO MATCH\n",
        "NO MATCH\n",
    ));
}

#[test]
fn full_program_exp05() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "BELLS\n",
            "TELLS\n",
            "FALLS\n",
            "DOLLS\n",
            "DULLS\n",
            "DOLLIES\n",
            "*ELLS\n",
            "*ELLS\n",
            "*ELLS\n",
            "*ELLS\n",
            "*ELLS\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "BELLS, TELLS\n",
        "BELLS, TELLS\n",
        "BELLS, TELLS\n",
        "BELLS, TELLS\n",
        "BELLS, TELLS\n",
    ));
}

#[test]
fn full_program_exp06() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "BELLS\n",
            "TELLS\n",
            "FALLS\n",
            "DOLLS\n",
            "DULLS\n",
            "DOLLIES\n",
            "D?LLS\n",
            "D?LLS\n",
            "D?LLS\n",
            "D?LLS\n",
            "D?LLS\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "DOLLS, DULLS\n",
        "DOLLS, DULLS\n",
        "DOLLS, DULLS\n",
        "DOLLS, DULLS\n",
        "DOLLS, DULLS\n",
    ));
}

#[test]
fn full_program_exp07() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "BELLS\n",
            "TELLS\n",
            "FALLS\n",
            "DOLLS\n",
            "DULLS\n",
            "DOLLIES\n",
            "D?LL*\n",
            "D?LL*\n",
            "D?LL*\n",
            "D?LL*\n",
            "D?LL*\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "DOLLS, DULLS, DOLLIES\n",
        "DOLLS, DULLS, DOLLIES\n",
        "DOLLS, DULLS, DOLLIES\n",
        "DOLLS, DULLS, DOLLIES\n",
        "DOLLS, DULLS, DOLLIES\n",
    ));
}

#[test]
fn full_program_exp08() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "BELLS\n",
            "TELLS\n",
            "FALLS\n",
            "DOLLS\n",
            "DULLS\n",
            "DOLLIES\n",
            "*LL*\n",
            "*LL*\n",
            "*LL*\n",
            "*LL*\n",
            "*LL*\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "BELLS, TELLS, FALLS, DOLLS, DULLS, DOLLIES\n",
        "BELLS, TELLS, FALLS, DOLLS, DULLS, DOLLIES\n",
        "BELLS, TELLS, FALLS, DOLLS, DULLS, DOLLIES\n",
        "BELLS, TELLS, FALLS, DOLLS, DULLS, DOLLIES\n",
        "BELLS, TELLS, FALLS, DOLLS, DULLS, DOLLIES\n",
    ));
}

#[test]
fn full_program_exp09() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "DS\n",
            "DDS\n",
            "FALLS\n",
            "DOLLS\n",
            "DULLS\n",
            "DOLLIES\n",
            "D*?S\n",
            "D**?S\n",
            "D***?S\n",
            "D*?*S\n",
            "D**?**S\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "DDS, DOLLS, DULLS, DOLLIES\n",
        "DDS, DOLLS, DULLS, DOLLIES\n",
        "DDS, DOLLS, DULLS, DOLLIES\n",
        "DDS, DOLLS, DULLS, DOLLIES\n",
        "DDS, DOLLS, DULLS, DOLLIES\n",
    ));
}

#[test]
fn full_program_exp10() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "DS\n",
            "DSS\n",
            "FALLS\n",
            "DOLLS\n",
            "DULLS\n",
            "DOLSIES\n",
            "D*S*S\n",
            "D*S*S\n",
            "D*S*S\n",
            "D*S*S\n",
            "D*S*S\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "DSS, DOLSIES\n",
        "DSS, DOLSIES\n",
        "DSS, DOLSIES\n",
        "DSS, DOLSIES\n",
        "DSS, DOLSIES\n",
    ));
}

#[test]
fn full_program_exp11() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n",
            "BELLS\n",
            "TELLS\n",
            "FALLS\n",
            "DOLLS\n",
            "DULLS\n",
            "DOLSDSED\n",
            "D*S*D\n",
            "D*S*D\n",
            "D*S*D\n",
            "D*S*D\n",
            "D*S*D\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "DOLSDSED\n",
        "DOLSDSED\n",
        "DOLSDSED\n",
        "DOLSDSED\n",
        "DOLSDSED\n",
    ));
}
