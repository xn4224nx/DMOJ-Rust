/*
 * COCI '15 Contest 4 #3 Deathstar
 * https://dmoj.ca/problem/coci15c4p3
 */

fn main() {
    let mut buffer = String::new();

    /* Read the size of the matrix. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mat_n = buffer.trim().parse::<usize>().unwrap();

    /* Keep a record of the results. */
    let mut results = vec![0; mat_n];

    /* Read the matrix line by line */
    for row_idx in 0..mat_n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let row: Vec<usize> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        /* OR each element of the row. */
        for col_idx in 0..mat_n {
            results[row_idx] |= row[col_idx];
        }
    }
    println!(
        "{}",
        results
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
