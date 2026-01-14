/*
 * CCC '17 S2 - High Tide, Low Tide
 * https://dmoj.ca/problem/ccc17s2
 */

fn main() {
    let mut buffer = String::new();

    /* Ignore the first number. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse the water heights. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut w_heights = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Order the values smallest to lowest. */
    w_heights.sort();

    /* Construct the order the values should be printed in. */
    let mut prnt_order = Vec::new();

    /* Even number of values */
    if w_heights.len() % 2 == 0 {
        for vl_idx in 0..(w_heights.len() / 2) {
            prnt_order.push(w_heights[w_heights.len() / 2 - vl_idx - 1]);
            prnt_order.push(w_heights[w_heights.len() / 2 + vl_idx]);
        }

    /* Odd number of values */
    } else {
        prnt_order.push(w_heights[w_heights.len() / 2]);
        for vl_idx in 0..(w_heights.len() / 2) {
            prnt_order.push(w_heights[w_heights.len() / 2 + vl_idx + 1]);
            prnt_order.push(w_heights[w_heights.len() / 2 - vl_idx - 1]);
        }
    }

    println!(
        "{}",
        prnt_order
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
