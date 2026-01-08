/*
 * CCC '17 S1 - Sum Game
 * https://dmoj.ca/problem/ccc17s1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of matches. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_days = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the match results. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let scores_0 = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let scores_1 = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Culmulatively sum the scores and determine the days when the scores are equal. */
    let mut total_0 = 0;
    let mut total_1 = 0;
    let mut num_same = 0;

    for day_idx in 0..num_days {
        total_0 += scores_0[day_idx];
        total_1 += scores_1[day_idx];

        if total_0 == total_1 {
            num_same = day_idx+1;
        }
    }
    println!("{}", num_same);
}
