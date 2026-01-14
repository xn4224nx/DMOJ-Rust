/*
 * CCC '21 S1 - Crazy Fencing
 * https://dmoj.ca/problem/ccc21s1
 */

fn main() {
    let mut buffer = String::new();

    /* Ignore the first line. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Read the panel heights. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let heights = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    /* Read the panel widths. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let widths = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    /* Calcuate the total area of the entire fence. */
    println!(
        "{}",
        (0..widths.len())
            .map(|x| widths[x] * (heights[x] + heights[x + 1]) / 2.0)
            .sum::<f64>()
    );
}
