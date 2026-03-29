/*
 * Educational DP Contest AtCoder E - Knapsack 2
 * https://dmoj.ca/problem/dpe
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of items and the capacity of the knapsack. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let meta = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut values = Vec::with_capacity(meta[0]);
    let mut weights = Vec::with_capacity(meta[0]);

    /* Read the potential items to put in the knapsack. */
    for _ in 0..meta[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let item_dt = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        values.push(item_dt[1]);
        weights.push(item_dt[0]);
    }
    println!("{}", knap_solver(meta[1], &values, &weights));
}

/// Solve the knapsack in O(Num * Items) time
fn knap_solver(capacity: usize, values: &Vec<usize>, weights: &Vec<usize>) -> usize {
    let total_val = values.iter().sum::<usize>();
    let mut feat_space = vec![capacity + 1; total_val + 1];

    /* The default value is an empty knapsack.   */
    feat_space[0] = 0;

    /* Overwrite values to save memory. */
    for itm_idx in 0..values.len() {
        for cap_mag in (values[itm_idx]..=total_val).rev() {
            feat_space[cap_mag] = std::cmp::min(
                feat_space[cap_mag],
                feat_space[cap_mag - values[itm_idx]] + weights[itm_idx],
            );
        }
    }

    /* Count backwards to find the best result. */
    return feat_space
        .into_iter()
        .enumerate()
        .rev()
        .find(|(_, cap_mag)| cap_mag <= &capacity)
        .unwrap_or((0, 0))
        .0;
}
