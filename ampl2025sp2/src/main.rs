/*
 * Amplitude Hackathon Summer '25 Problem 2 - AI (Also Easy)
 * https://dmoj.ca/problem/ampl2025sp2
 */

fn main() {
    let mut buffer = String::new();

    /* How many words are there. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_words = buffer.trim_end().parse::<usize>().unwrap();

    /* Do the words contain the substring "AI"? */
    for _ in 0..num_words {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Keep track of the last letter seen. */
        let mut last_char = ' ';
        let mut patt_seen = false;

        /* Find one instance of I following A, ignoring other chars. */
        for w_char in buffer.chars() {
            if w_char == 'A' || w_char == 'I' {
                if last_char == 'A' && w_char == 'I' {
                    patt_seen = true;
                    break;
                }

                last_char = w_char;
            }
        }

        println!(
            "{}",
            match patt_seen {
                true => "YES",
                false => "NO",
            }
        );
    }
}
