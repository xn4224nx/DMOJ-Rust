/*
 * COCI '15 Contest 2 #1 Marko
 * https://dmoj.ca/problem/coci15c2p1
 */

use std::collections::HashMap;

fn main() {
    let keypad = HashMap::from([
        ('a', '2'),
        ('b', '2'),
        ('c', '2'),
        ('d', '3'),
        ('e', '3'),
        ('f', '3'),
        ('g', '4'),
        ('h', '4'),
        ('i', '4'),
        ('j', '5'),
        ('k', '5'),
        ('l', '5'),
        ('m', '6'),
        ('n', '6'),
        ('o', '6'),
        ('p', '7'),
        ('q', '7'),
        ('r', '7'),
        ('s', '7'),
        ('t', '8'),
        ('u', '8'),
        ('v', '8'),
        ('w', '9'),
        ('x', '9'),
        ('y', '9'),
        ('z', '9'),
    ]);
    let mut buffer = String::new();

    /* Read the dictionary size. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dict_len = buffer.trim().parse::<usize>().unwrap();

    /* Read the dictionary words and convert to keystrokes. */
    let mut dict: Vec<String> = Vec::with_capacity(dict_len);
    for _ in 0..dict_len {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        dict.push(
            buffer
                .trim()
                .chars()
                .map(|x| keypad.get(&x).unwrap())
                .collect(),
        );
    }

    /* Read the actual key strokes. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let true_keys: String = buffer.trim().chars().collect();

    /* Check to see the number of words that can be printed. */
    println!("{}", dict.into_iter().filter(|x| *x == true_keys).count());
}
