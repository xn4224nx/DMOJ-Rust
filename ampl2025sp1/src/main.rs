/*
 * Amplitude Hackathon Summer '25 Problem 1 - AI (Easy)
 * https://dmoj.ca/problem/ampl2025sp1
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
        println!(
            "{}",
            match buffer.contains("AI") {
                true => "YES",
                false => "NO",
            }
        );
    }
}
