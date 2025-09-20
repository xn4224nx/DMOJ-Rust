/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin("coci06c3p3").unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "...D\n", "..C.\n", ".B..\n", "A...\n",))
        .assert();
    assert.success().stdout("4\n");
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin("coci06c3p3").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n", "..T..\n", "A....\n", ".FE.R\n", "....X\n", "S....\n",
        ))
        .assert();
    assert.success().stdout("3\n");
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin("coci06c3p3").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "10\n",
            "....AB....\n",
            "..C....D..\n",
            ".E......F.\n",
            "...G..H...\n",
            "I........J\n",
            "K........L\n",
            "...M..N...\n",
            ".O......P.\n",
            "..Q....R..\n",
            "....ST....\n",
        ))
        .assert();
    assert.success().stdout("0\n");
}
