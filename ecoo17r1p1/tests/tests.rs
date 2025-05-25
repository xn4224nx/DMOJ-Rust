/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ecoo17r1p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4000\n",
            "0.5 0.2 0.1 0.2\n",
            "400\n",
            "6000\n",
            "0.1 0.1 0.45 0.35\n",
            "2000\n",
            "4000\n",
            "0.5 0.2 0.1 0.2\n",
            "400\n",
            "6000\n",
            "0.1 0.1 0.45 0.35\n",
            "2000\n",
            "4000\n",
            "0.5 0.2 0.1 0.2\n",
            "400\n",
            "6000\n",
            "0.1 0.1 0.45 0.35\n",
            "2000\n",
            "4000\n",
            "0.5 0.2 0.1 0.2\n",
            "400\n",
            "6000\n",
            "0.1 0.1 0.45 0.35\n",
            "2000\n",
            "4000\n",
            "0.5 0.2 0.1 0.2\n",
            "400\n",
            "6000\n",
            "0.1 0.1 0.45 0.35\n",
            "2000\n",
        ))
        .assert();
    assert
        .success()
        .stdout("YES\nNO\nYES\nNO\nYES\nNO\nYES\nNO\nYES\nNO\n");
}
