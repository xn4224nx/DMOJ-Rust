/*
 * CCC '11 J2 - Who Has Seen The Wind
 * https://dmoj.ca/problem/ccc11j2
 */

fn main() {
    let mut buffer = String::new();

    /* What are the descent conditions. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let humidity = buffer.trim_end().parse::<i32>().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let max_time = buffer.trim_end().parse::<i32>().unwrap();

    for t_idx in 1..=max_time {
        if bal_altitude(humidity, t_idx) <= 0 {
            println!("The balloon first touches ground at hour:\n{}", t_idx);
            return;
        }
    }
    println!("The balloon does not touch ground in the given time.");
}

fn bal_altitude(humidity: i32, time: i32) -> i32 {
    -6 * time * time * time * time + humidity * time * time * time + 2 * time * time + time
}
