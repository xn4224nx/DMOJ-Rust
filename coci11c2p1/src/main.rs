/*
 * COCI '11 Contest 2 #1 Najboljih 5
 * https://dmoj.ca/problem/coci11c2p1
 */

const NUM_SCORES: usize = 8;
const NUM_HIGHEST: usize = 5;

fn main() {
    let mut buffer = String::new();
    let mut scores: Vec<usize> = Vec::with_capacity(NUM_SCORES);
    let mut score_idxs: Vec<usize> = Vec::with_capacity(NUM_SCORES);

    /* Read the scores */
    for idx in 0..NUM_SCORES {
        std::io::stdin().read_line(&mut buffer).unwrap();
        scores.push(buffer.trim_end().parse::<usize>().unwrap());
        score_idxs.push(idx);
        buffer.clear();
    }

    /* Sort the scores largest to smallest. */
    let mut ordered: Vec<_> = score_idxs.into_iter().zip(scores).collect();
    ordered.sort_by_key(|&(_, k)| k);
    ordered.reverse();

    /* The players total score will be the sum of the NUM_HIGHEST scores. */
    let mut used_idxs: Vec<String> = Vec::new();
    let mut total_score = 0;
    for idx in 0..NUM_HIGHEST {
        total_score += ordered[idx].1;
        used_idxs.push((ordered[idx].0 + 1).to_string());
    }

    /* Show the results. */
    used_idxs.sort();
    println!("{}\n{}", total_score, used_idxs.join(" "));
}
