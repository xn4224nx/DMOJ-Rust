/*
 * Mispelling
 * https://dmoj.ca/problem/a1
 */

fn main() {
    let mut buffer = String::new();

    /* How many words will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_words = buffer.trim_end().parse::<usize>().unwrap();

    for wrd_idx in 0..num_words {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let (raw_num, raw_words) = buffer.trim_end().split_once(" ").unwrap();

        /* What is the index of the letter to be replaced? */
        let rm_idx = raw_num.parse::<usize>().unwrap() - 1;

        println!(
            "{} {}",
            wrd_idx + 1,
            raw_words
                .chars()
                .enumerate()
                .filter(|(x, _)| *x != rm_idx)
                .map(|(_, y)| y)
                .collect::<String>()
        );
    }
}
