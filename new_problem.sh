#!/bin/bash
# Create a new cargo project for a DMOJ problem.

# This script requires the name of the new project.
if [[ $# -ne 1 ]] then
    echo "USAGE: $0 problem_code"
    exit 0
fi


# Ensure project folder does not already exist.
if [ -d "$1" ]; then
    echo "Problem $1 already exists!"
    exit 0
fi

# Create the new project
cargo new $1

# Perform tasks within the project directory
cd ./$1

# Add in the development module
cargo add --dev assert_cmd

# Add in the testing file and directory.
mkdir tests
touch ./tests/tests.rs

# Add in the testing script
echo """/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!(


    )).assert();
    assert.success().stdout(concat!(


    ));
}""" > ./tests/tests.rs
