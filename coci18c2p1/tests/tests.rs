/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("coci18c2p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n", "10\n", "1400\n", "1500\n", "2\n", "7\n", "2000\n",
        ))
        .assert();
    assert.success().stdout("3\n1\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("coci18c2p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6\n", "15\n", "30\n", "35\n", "55\n", "60\n", "2065\n", "7\n", "20\n", "25\n", "40\n",
            "45\n", "50\n", "2070\n", "2075\n",
        ))
        .assert();
    assert.success().stdout("10\n5\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("coci18c2p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "11\n", "1402\n", "1412\n", "1428\n", "1430\n", "1441\n", "1444\n", "1453\n", "1483\n",
            "1485\n", "1489\n", "1490\n", "9\n", "1403\n", "1405\n", "1409\n", "1435\n", "1459\n",
            "1460\n", "1461\n", "1487\n", "1495\n",
        ))
        .assert();
    assert.success().stdout("8\n2\n");
}
