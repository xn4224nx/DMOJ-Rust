/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc14j1").unwrap();
    let assert = cmd.write_stdin(concat!("60\n", "70\n", "50\n",)).assert();
    assert.success().stdout(concat!("Scalene\n",));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc14j1").unwrap();
    let assert = cmd.write_stdin(concat!("60\n", "75\n", "55\n",)).assert();
    assert.success().stdout(concat!("Error\n",));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc14j1").unwrap();
    let assert = cmd.write_stdin(concat!("60\n", "60\n", "60\n",)).assert();
    assert.success().stdout(concat!("Equilateral\n",));
}

#[test]
fn exp_04() {
    let mut cmd = Command::cargo_bin("ccc14j1").unwrap();
    let assert = cmd.write_stdin(concat!("55\n", "60\n", "55\n",)).assert();
    assert.success().stdout(concat!("Error\n",));
}

#[test]
fn exp_05() {
    let mut cmd = Command::cargo_bin("ccc14j1").unwrap();
    let assert = cmd.write_stdin(concat!("55\n", "70\n", "55\n",)).assert();
    assert.success().stdout(concat!("Isosceles\n",));
}
