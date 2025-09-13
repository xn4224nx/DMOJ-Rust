/*
 * K-th Minimum Number
 * https://dmoj.ca/problem/kthminnumber
 */

fn main() {
    let mut answer = 0;
    let mut buffer = String::new();

    /* Read the metadata. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let metadata = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Read the array of numbers and preserve its original index. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut numbers = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .enumerate()
        .collect::<Vec<(usize, usize)>>();

    /* Sort the given array. */
    numbers.sort_by(|a, b| a.1.cmp(&b.1));

    /* Read the encrypted queries in the form l r k. */
    for _ in 0..metadata[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let query = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| (x.parse::<usize>().unwrap() ^ answer) - 1)
            .collect::<Vec<usize>>();

        /* Find the relevant numbers. */
        let mut small_cnt = 0;
        for (idx, val) in numbers.iter() {
            if query[0] <= *idx && *idx <= query[1] {
                if small_cnt == query[2] {
                    answer = *val;
                    println!("{}", answer);
                    break;
                } else {
                    small_cnt += 1;
                }
            }
        }
    }
}
