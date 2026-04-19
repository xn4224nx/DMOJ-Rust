/*
 * SAC '22 Code Challenge 3 Junior P3 - Normal Probabilities
 * https://dmoj.ca/problem/sac22cc3jp3
 */

fn main() {
    let mut buffer = String::new();
    let mut total_prob: f64 = 1.0;

    /* How many events will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_events = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the events and calculate the total probability. */
    for _ in 0..num_events {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        total_prob *= match buffer.chars().next().unwrap() {
            'B' => 0.8,
            'C' => 0.6,
            'D' => 0.4,
            'E' => 0.2,
            _ => 1.0,
        };
    }
    println!("{:.06}", total_prob);
}
