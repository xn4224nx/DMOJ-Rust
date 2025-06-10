/*
 * CCC '13 S1 - From 1987 to 2013
 * https://dmoj.ca/problem/ccc13s1
 */

use std::collections::HashSet;

fn main() {
    let mut start_num = read_num() + 1;

    while duplicate_digits(start_num) {
        start_num += 1;
    }
    println!("{}", start_num);
}

fn read_num() -> usize {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().parse::<usize>().unwrap();
}

fn duplicate_digits(num: usize) -> bool {
    let raw_nums: Vec<char> = num.to_string().chars().collect();
    let dist_digits: HashSet<char> = raw_nums.iter().cloned().collect();
    return raw_nums.len() != dist_digits.len();
}
