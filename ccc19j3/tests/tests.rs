/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc19j3").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "+++===!!!!\n",
            "777777......TTTTTTTTTTTT\n",
            "(AABBC)\n",
            "3.1415555\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "3 + 3 = 4 !\n",
        "6 7 6 . 12 T\n",
        "1 ( 2 A 2 B 1 C 1 )\n",
        "1 3 1 . 1 1 1 4 1 1 4 5\n",
    ));
}
