/*
 * CCC '98 S1 - Censor
 * https://dmoj.ca/problem/ccc98s1
 */

const CENSOR_LEN: usize = 4;
const CENSOR_REPL: char = '*';

fn main() {
    let mut buffer = String::new();

    /* How many lines of text will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_lines = buffer.trim_end().parse::<usize>().unwrap();

    for _ in 0..num_lines {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Replace words of a certain length. */
        println!(
            "{}",
            buffer
                .trim_end()
                .split_whitespace()
                .map(|x| if x.len() == CENSOR_LEN {
                    vec![CENSOR_REPL; CENSOR_LEN].iter().collect::<String>()
                } else {
                    x.to_string()
                })
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
