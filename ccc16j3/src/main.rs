/*
 * CCC '16 J3 - Hidden Palindrome
 * https://dmoj.ca/problem/ccc16j3
 */

fn main() {
    let mut longest_palin = 1; /* A single letter counts as a palindrome. */
    let mut buffer = String::new();

    /* Read the word. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let word = buffer.trim_end().chars().collect::<Vec<char>>();

    /* Find all the palinromes and record the length of the longest.   */
    for s_idx in 0..(word.len() - 1) {
        for e_idx in (s_idx + 1)..=word.len() {
            let slc_len = e_idx - s_idx;

            if slc_len > longest_palin && is_palindrome(&word[s_idx..e_idx]) {
                longest_palin = slc_len;
            }
        }
    }
    println!("{}", longest_palin);
}

fn is_palindrome(sub_word: &[char]) -> bool {
    for w_idx in 0..sub_word.len() / 2 {
        if sub_word[w_idx] != sub_word[sub_word.len() - 1 - w_idx] {
            return false;
        }
    }
    return true;
}
