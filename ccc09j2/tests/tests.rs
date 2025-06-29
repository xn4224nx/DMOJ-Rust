/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc09j2").unwrap();
    let assert = cmd.write_stdin("1\n2\n3\n2\n").assert();
    assert.success().stdout(concat!(
        "0 Brown Trout, 1 Northern Pike, 0 Yellow Pickerel\n",
        "1 Brown Trout, 0 Northern Pike, 0 Yellow Pickerel\n",
        "2 Brown Trout, 0 Northern Pike, 0 Yellow Pickerel\n",
        "Number of ways to catch fish: 3\n",
    ));
}
