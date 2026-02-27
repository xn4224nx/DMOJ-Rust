/*
 * CCC '26 J2 - Olympic Scores
 * https://dmoj.ca/problem/ccc26j2
 */

const NUM_SCORES: usize = 5;

fn main() {
    let mut buffer = String::new();

    /* Read the scores. */
    for _ in 0..NUM_SCORES {
        std::io::stdin().read_line(&mut buffer).unwrap();
    }

    /* Parse the inputted values. */
    let mut scores = buffer
        .trim_end()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    assert_eq!(scores.len(), NUM_SCORES);

    /* Read the difficulty factor. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let diff_factor = buffer.trim_end().parse::<usize>().unwrap();

    scores.sort();

    /* What is the athletes overall score? */
    println!("{}", (scores[1] + scores[2] + scores[3]) * diff_factor);
}
