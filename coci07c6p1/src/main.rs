/*
 * COCI '07 Contest 6 #1 Parking
 * https://dmoj.ca/problem/coci07c6p1
 */

fn main() {
    let mut total_cost = 0;
    let mut buffer = String::new();

    /* Read the price of parking */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let prices = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Read the truck start and end times. */
    let mut truck_times = Vec::with_capacity(prices.len());
    for _ in 0..prices.len() {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        truck_times.push(
            buffer
                .trim_end()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    /* Determine the cost of parking the trucks minute by minute. */
    let mut num_trucks = 0;
    for time_idx in 1..=100 {
        /* Record the trucks arriving */
        for trk_idx in 0..prices.len() {
            if truck_times[trk_idx][0] == time_idx {
                num_trucks += 1;
            }
        }

        /* Record the trucks leaving. */
        for trk_idx in 0..prices.len() {
            if truck_times[trk_idx][1] == time_idx {
                num_trucks -= 1;
            }
        }

        /* Determine the cost of this minute. */
        if num_trucks > 0 {
            total_cost += prices[num_trucks - 1] * num_trucks;
        }
    }
    println!("{}", total_cost);
}
