/*
 * CCC '16 S1 - Ragaman
 * https://dmoj.ca/problem/ccc16s1
 */

use std::collections::HashMap;

const CORRECT: char = 'A';
const WRONG: char = 'N';

fn main() {
    let (original, orig_len) = read_word();
    let (anagram, anag_len) = read_word();

    /* Both words must be the same length. */
    if orig_len != anag_len {
        println!("{}", WRONG);
    } else {
        println!("{}", possible_anagram(&original, &anagram));
    }
}

fn read_word() -> (HashMap<char, usize>, usize) {
    let mut letter_cnt = HashMap::new();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Record the instances of each letter */
    for letter in buffer.chars() {
        if letter.is_ascii_alphabetic() {
            letter_cnt
                .entry(letter)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }
    return (letter_cnt, buffer.len());
}

fn possible_anagram(orig: &HashMap<char, usize>, jumb: &HashMap<char, usize>) -> char {
    let wildcards = orig.values().sum::<usize>() - jumb.values().sum::<usize>();

    /* Count the missing letters. */

    if wildcards == 0 {
        for (letter, letter_cnt) in orig.iter() {
            if !jumb.contains_key(letter) || jumb.get(&letter).unwrap() != letter_cnt {
                return WRONG;
            }
        }
        return CORRECT;
    } else {
        for (letter, letter_cnt) in jumb.iter() {
            if !orig.contains_key(letter) || orig.get(&letter).unwrap() < letter_cnt {
                return WRONG;
            }
        }
        return CORRECT;
    }
}
