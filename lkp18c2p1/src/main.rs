/*
 * LKP '18 Contest 2 P1 - Food Lines
 * https://dmoj.ca/problem/lkp18c2p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of new people going to the queue. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let specs = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Read the current queue sizes. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut queues = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Print the shortest queue and the increment it. */
    for _ in 0..specs[1] {
        let idx_of_min_q = (0..queues.len()).min_by_key(|x| queues[*x]).unwrap();
        println!("{}", queues[idx_of_min_q]);
        queues[idx_of_min_q] += 1;
    }
}
