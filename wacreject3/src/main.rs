/*
 * Wesley's Anger Contest Reject 2 - carrot suba
 * https://dmoj.ca/problem/wacreject3
 */

fn main() {
    let mut buffer = String::new();

    /* How many problems are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_probs = buffer.trim_end().parse::<usize>().unwrap();

    /* Read and solve the problems. */
    for _ in 0..num_probs {
        buffer.clear();

        /* Read the values. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let in_vals = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Determine the degree of the new polynomial.  */
        println!("{}", in_vals[0] + in_vals[1] - 1);
    }
}
