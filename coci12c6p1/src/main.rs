/*
 * COCI '12 Contest 6 #1 Baka
 * https://dmoj.ca/problem/coci12c6p1
 */

fn main() {
    let mut buffer = String::new();
    let mut phone_number = 0;

    /* Read the word. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Determine the number it represents. */
    for wrd_val in buffer.chars() {
        phone_number += match wrd_val {
            'A' | 'B' | 'C' => 3,
            'D' | 'E' | 'F' => 4,
            'G' | 'H' | 'I' => 5,
            'J' | 'K' | 'L' => 6,
            'M' | 'N' | 'O' => 7,
            'P' | 'Q' | 'R' | 'S' => 8,
            'T' | 'U' | 'V' => 9,
            'W' | 'X' | 'Y' | 'Z' => 10,
            _ => 0,
        }
    }
    println!("{}", phone_number);
}
