/*
 * COCI '08 Contest 3 #2 Kemija
 * https://dmoj.ca/problem/coci08c3p2
 */

fn main() {
    let mut enc_idx = 0;
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    /* Read the encoded sentence */
    let mut sentence = String::new();
    std::io::stdin().read_line(&mut sentence).unwrap();

    /* Convert the sentence to vector of chars */
    let enc_chars: Vec<char> = sentence.chars().collect();

    /* Decode the sentence. */
    while enc_idx < enc_chars.len() {
        print!("{}", enc_chars[enc_idx]);

        /* If the current char is a vowle skip printing the next two chars */
        if vowels.contains(&enc_chars[enc_idx]) {
            enc_idx += 3;
        } else {
            enc_idx += 1;
        }
    }
}
