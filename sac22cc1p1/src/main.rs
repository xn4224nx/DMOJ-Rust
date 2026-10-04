/*
 * SAC '22 Code Challenge 1 P1 - That Teacher
 * https://dmoj.ca/problem/sac22cc1p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the problem data. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse the problem data. */
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    assert_eq!(data.len(), 3);

    /* How many candy bars he will be left. */
    println!("{}", data[2].saturating_sub(data[0] * data[1]));
}
