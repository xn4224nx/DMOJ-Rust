/*
 * CCC '07 J4 - Anagram Checker
 * https://dmoj.ca/problem/ccc07j4
 */

use std::collections::HashMap;

fn main() {
    println!(
        "{}",
        if read_wrd_count() == read_wrd_count() {
            "Is an anagram."
        } else {
            "Is not an anagram."
        }
    );
}

/// Read a line of input and convert it to a hashmap of the constituant letters.
fn read_wrd_count() -> HashMap<char, usize> {
    let mut wrd_cnt = HashMap::new();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Count each letter in the  */
    for letter in buffer.trim().chars() {
        if letter.is_ascii_uppercase() {
            wrd_cnt.entry(letter).and_modify(|x| *x += 1).or_insert(1);
        }
    }
    return wrd_cnt;
}
