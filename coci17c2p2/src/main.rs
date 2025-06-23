/*
 * COCI '17 Contest 2 #2 ZigZag
 * https://dmoj.ca/problem/coci17c2p2
 */

use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();

    /* Read the number of words and letters. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    /* Store a record of the words and how many times it has been used. */
    let mut words: HashMap<char, Vec<String>> = HashMap::new();
    let mut next_word_idx: HashMap<char, usize> = HashMap::new();

    /* Read the words */
    for _ in 0..data[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let leading_letter: char = buffer.chars().next().unwrap();

        /* If this character doesn't have an entry. */
        if !words.contains_key(&leading_letter) {
            words.insert(leading_letter, Vec::new());
        }

        /* Save details of this word. */
        words
            .get_mut(&leading_letter)
            .unwrap()
            .push(buffer.trim().to_string());
    }

    /* Sort the lists of words for each letter. */
    for (letter, le_words) in words.iter_mut() {
        le_words.sort();

        /* Keep a record of the next word to choose. */
        next_word_idx.insert(*letter, 0);
    }

    /* Read the letters and play the game. */
    for _ in 0..data[1] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let choice: char = buffer.chars().next().unwrap();

        /* What is the index of the word to be chosen. */
        let curr_wrd_idx = *next_word_idx.get(&choice).unwrap();

        /* What word should be used? */
        println!("{}", words.get(&choice).unwrap()[curr_wrd_idx]);

        /* Update the index of the next letter for that char. */
        let num_words = words.get(&choice).unwrap().len();
        *next_word_idx.get_mut(&choice).unwrap() = (curr_wrd_idx + 1) % num_words;
    }
}
