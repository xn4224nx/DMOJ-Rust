/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd.write_stdin("1\n1(2)\n").assert();
    assert.success().stdout("YES\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd.write_stdin("1\n(1)(2)\n").assert();
    assert.success().stdout("YES\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd.write_stdin("1\n((1))(2)\n").assert();
    assert.success().stdout("YES\n");
}

#[test]
fn exp_04() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd.write_stdin("1\n(500())\n").assert();
    assert.success().stdout("YES\n");
}

#[test]
fn exp_05() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd.write_stdin("1\n(12\n").assert();
    assert.success().stdout("NO\n");
}

#[test]
fn exp_06() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd.write_stdin("1\n(1))\n").assert();
    assert.success().stdout("NO\n");
}

#[test]
fn exp_07() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd.write_stdin("1\n((1)()\n").assert();
    assert.success().stdout("NO\n");
}

#[test]
fn exp_08() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd.write_stdin("1\n)(\n").assert();
    assert.success().stdout("NO\n");
}

#[test]
fn exp_09() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd.write_stdin("1\n\n").assert();
    assert.success().stdout("YES\n");
}

#[test]
fn exp_10() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd.write_stdin("1\n()\n").assert();
    assert.success().stdout("YES\n");
}

#[test]
fn exp_11() {
    let mut cmd = Command::cargo_bin("dmopc19c4p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "7\n",
            "1(2)\n",
            "(1)(2)\n",
            "((1))(2)\n",
            "(500())\n",
            "(12\n",
            "(1))\n",
            "((1)()\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "YES\n", "YES\n", "YES\n", "YES\n", "NO\n", "NO\n", "NO\n",
    ));
}
