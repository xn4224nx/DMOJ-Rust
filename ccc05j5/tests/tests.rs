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
            "A\n",
            "ANA\n",
            "ANANA\n",
            "BANANAS\n",
            "BANANA\n",
            "X\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("YES\n", "YES\n", "YES\n", "YES\n", "NO\n",));
}

#[test]
fn full_program_exp01() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "ANA\n",
            "BS\n",
            "AANA\n",
            "BBANASS\n",
            "BANASNANANA\n",
            "ANBANANAS\n",
            "BANANANANANANBANANANANANBANANASNANANANBANANASNANANANSS\n",
            "SAB\n",
            "X\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "YES\n", "NO\n", "NO\n", "YES\n", "YES\n", "YES\n", "NO\n", "NO\n",
    ));
}

#[test]
fn full_program_exp02() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("BANANASNBANANAS\n", "\n", "BBBBASSSS\n", "X\n",))
        .assert();
    assert.success().stdout(concat!("YES\n", "NO\n", "YES\n"));
}

#[test]
fn full_program_exp03() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp04() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("A\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp05() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("ANA\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp06() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("ANANA\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp07() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BANANAS\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp08() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BANANA\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp09() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BS\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp10() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("AANA\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp11() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BBANASS\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp12() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BANASNANANA\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp13() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("ANBANANAS\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp14() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "BANANANANANANBANANANANANBANANASNANANANBANANASNANANANSS\n",
            "X\n",
        ))
        .assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp15() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("SAB\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp16() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "BANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANA",
            "NANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANA",
            "NANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANA",
            "NANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANA",
            "NANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANA",
            "NANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANA",
            "NANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANA",
            "NANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANANA",
            "NANANANANANANANANANANANANANANANANANANANANANANANANANANANANANS\n",
            "X\n",
        ))
        .assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp17() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "BANANANANANANBANANANANANBANANASNANANANBANANASNANANANSS\n",
            "X\n",
        ))
        .assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp18() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("BANANASNBANANAS\n", "X\n",))
        .assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp19() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BBBBASSSS\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp20() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BANSA\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp21() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("ANABANANAS\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp22() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("BANANASNBANANAS\n", "X\n",))
        .assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp23() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BBANANASS\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp24() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("ANAANA\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp25() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BANS\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp26() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("AN\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp27() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BANAS\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp28() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BANASBANAS\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp29() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("SS\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp30() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BB\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp31() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("ANSS\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp32() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BSANABS\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}

#[test]
fn full_program_exp33() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("BANASNBANAS\n", "X\n",)).assert();
    assert.success().stdout(concat!("YES\n",));
}

#[test]
fn full_program_exp34() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("SABAS\n", "X\n",)).assert();
    assert.success().stdout(concat!("NO\n",));
}
