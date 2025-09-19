/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("cco25p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "6 10\n", "1 1\n", "5 2\n", "200 6\n", "9 2\n", "6 2\n", "100 1\n",
        ))
        .assert();
    assert.success().stdout(concat!("310\n"));
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("cco25p1").unwrap();
    let assert = cmd
        .write_stdin(concat!("4 16\n", "3 1 \n", "2 8\n", "5 16\n", "4 8\n",))
        .assert();
    assert.success().stdout(concat!("7\n"));
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("cco25p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1 576460752303423487\n",
            "1000000000 576460752303423488\n",
        ))
        .assert();
    assert.success().stdout(concat!("0\n"));
}

#[test]
fn exp_04() {
    let mut cmd = Command::cargo_bin("cco25p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "15 23752394551518\n",
            "457687868 4398046511104 \n",
            "527769950 4398046511104\n",
            "336200204 2199023255552\n",
            "555090674 4398046511104\n",
            "452384 4294967296\n",
            "697175056 4398046511104\n",
            "62269946 274877906944\n",
            "24293218 549755813888\n",
            "214670437 1099511627776\n",
            "6306990 137438953472\n",
            "103832403 549755813888\n",
            "205334902 2199023255552\n",
            "20326312 137438953472\n",
            "4723060 34359738368\n",
            "176783630 4398046511104\n",
        ))
        .assert();
    assert.success().stdout(concat!("3102936938\n"));
}
