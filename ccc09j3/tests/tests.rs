/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc09j3").unwrap();
    let assert = cmd.write_stdin("1300\n").assert();
    assert.success().stdout(concat!(
        "1300 in Ottawa\n",
        "1000 in Victoria\n",
        "1100 in Edmonton\n",
        "1200 in Winnipeg\n",
        "1300 in Toronto\n",
        "1400 in Halifax\n",
        "1430 in St. John's\n",
    ));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc09j3").unwrap();
    let assert = cmd.write_stdin("45\n").assert();
    assert.success().stdout(concat!(
        "45 in Ottawa\n",
        "2145 in Victoria\n",
        "2245 in Edmonton\n",
        "2345 in Winnipeg\n",
        "45 in Toronto\n",
        "145 in Halifax\n",
        "215 in St. John's\n",
    ));
}
