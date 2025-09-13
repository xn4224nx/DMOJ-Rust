/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dwite08c3p3").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "10 oz in gill\n",
            "20 oz in pt\n",
            "2 gal in oz\n",
            "1 gal in qt\n",
            "2 pt in pt\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("2\n", "1\n", "320\n", "4\n", "2\n",));
}
