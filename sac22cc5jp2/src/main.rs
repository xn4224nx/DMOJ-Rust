/*
 * SAC '22 Code Challenge 5 Junior P2 - Badminton Scoring
 * https://dmoj.ca/problem/sac22cc5jp2
 */

fn main() {
    let mut buffer = String::new();
    let mut total_wins = 0;

    /* How many matches are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_matches = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the match scores. */
    for _ in 0..num_matches {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let scores = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if scores[0] > scores[1] {
            total_wins += 1;
        }
    }
    println!("{}", total_wins);
}
