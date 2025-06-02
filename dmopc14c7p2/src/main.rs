/*
 * DMOPC '14 Contest 7 P2 - Tides
 * https://dmoj.ca/problem/dmopc14c7p2
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();

    /* Read the wave heights. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let heights: Vec<u32> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    /* Find the maximum and minimum heights and their locations. */
    let mut max_h = 0;
    let mut min_h = u32::MAX;
    let mut max_idx = 0;
    let mut min_idx = 0;

    for wv_idx in 0..heights.len() {
        if heights[wv_idx] > max_h {
            max_h = heights[wv_idx];
            max_idx = wv_idx;
        }

        if heights[wv_idx] < min_h {
            min_h = heights[wv_idx];
            min_idx = wv_idx;
        }
    }

    /* Make sure the lowest index is the start and highest is the end. */
    if max_idx < min_idx {
        (max_idx, min_idx) = (min_idx, max_idx);
    }

    /* Check every value between the min and max. */
    for wv_idx in min_idx+1..max_idx+1 {
        if heights[wv_idx] <= heights[wv_idx - 1] {
            println!("unknown");
            return;
        }
    }

    /* If the data is valid print the absolute difference. */
    println!("{}", max_h - min_h);
}
