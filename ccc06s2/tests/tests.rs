/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc06s2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG\n",
            "UIFARVJDLACSPXOAGPYAKVNQTAPWFSAUIFAMB ZAEPH\n",
            "XFABSFAWFSZACBEAQFPQMFAEPJOHAWFSZACBEAUIJOHTAIBAIB\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "WE ARE VERY BAD PEOPLE DOING VERY BAD THINGS HA HA\n",
    ));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc06s2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "THERE ARE NOT ENOUGH LETTERS\n",
            "XQAZASEZASNYXSANYLWQSTAXXAZM\n",
            "JSCENNYXSIACYIASXQJM\n",
        ))
        .assert();
    assert.success().stdout(concat!(". .ANNOT .E.O.E TH.S\n",));
}
