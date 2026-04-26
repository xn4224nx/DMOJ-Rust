/*
 * DMOPC '19 Contest 5 P0 - Concurrent Competitor Counting
 * https://dmoj.ca/problem/dmopc19c5p0
 */

fn main() {
    let mut buffer = String::new();

    /* Read the problem data. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Deteremine who will advance. */
    for _ in 0..data[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let (name, rw_score) = buffer.trim_end().split_once(' ').unwrap();
        let score = rw_score.parse::<usize>().unwrap();

        /* Show the result. */
        println!(
            "{} {}",
            name,
            if score > data[1] {
                "will advance"
            } else {
                "will not advance"
            }
        );
    }
}
