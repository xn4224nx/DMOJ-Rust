/*
 * CCC '16 S2 - Tandem Bicycle
 * https://dmoj.ca/problem/ccc16s2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the problem type. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let problem_type = buffer.trim().parse::<usize>().unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Read the A cyclist speeds. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut cycl_a = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Read the B cyclist speeds. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut cycl_b = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Sort the cyclists by speed. */
    cycl_a.sort();
    cycl_b.sort();

    /* Problem type 2 means the slowest riders are matched with the fastest. */
    if problem_type == 2 {
        cycl_b.reverse();
    }

    /* Determine the maximum speed of each pair of cyclist. */
    println!(
        "{}",
        (0..cycl_b.len())
            .map(|x| std::cmp::max(cycl_a[x], cycl_b[x]))
            .sum::<usize>()
    );
}
