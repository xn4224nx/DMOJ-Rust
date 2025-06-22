/*
 * CCO '99 P2 - Common Words
 * https://dmoj.ca/problem/cco99p2
 */

use std::cmp::Reverse;
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();

    /* Read the number of corpuses. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_corpuses = buffer.trim().parse::<usize>().unwrap();

    /* For each corpus find the x most common word*/
    for corpus_idx in 0..num_corpuses {
        let mut word_cnts: HashMap<String, usize> = HashMap::new();

        /* Read the word count and the target common word. */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let data: Vec<usize> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        /* Read the words. */
        for _ in 0..data[0] {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
            buffer.pop();

            /* Record the counts of each word. */
            word_cnts
                .entry(buffer.clone())
                .and_modify(|x| *x += 1)
                .or_insert(0);
        }

        /* Extract all words and their counts */
        let mut all_wrds: Vec<(String, usize)> = word_cnts.drain().map(|(k, v)| (k, v)).collect();

        /* Sort the words */
        all_wrds.sort_by_key(|(_k, v)| Reverse(*v));

        /* Print the title. */
        println!(
            "{}{} most common word(s):",
            data[1],
            ordinal_display(&data[1])
        );

        /* Print the applicable words if they exist. */
        let mut curr_rnk = 1;
        for wrd_idx in 0..all_wrds.len() {
            /* Is this the rank of word we are looking for? */
            if curr_rnk == data[1] {
                println!("{}", all_wrds[wrd_idx].0);
            }

            /* See if the next word has the same count as this word. */
            if wrd_idx != all_wrds.len() - 1 && all_wrds[wrd_idx].1 != all_wrds[wrd_idx + 1].1 {
                curr_rnk += 1;
            }
        }

        if corpus_idx < num_corpuses - 1 {
            println!();
        }
    }
}

pub fn ordinal_display(xth: &usize) -> String {
    let digits: Vec<u32> = xth
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    if *xth < 10 {
        return if *xth == 1 {
            String::from("st")
        } else if *xth == 2 {
            String::from("nd")
        } else if *xth == 3 {
            String::from("rd")
        } else {
            String::from("th")
        };
    } else if digits[digits.len() - 2] == 1 {
        return String::from("th");
    } else if digits[digits.len() - 1] == 1 {
        return String::from("st");
    } else if digits[digits.len() - 1] == 2 {
        return String::from("nd");
    } else if digits[digits.len() - 1] == 3 {
        return String::from("rd");
    } else {
        return String::from("th");
    };
}
