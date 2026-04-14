/*
 * ECOO '21 P1 - Many Messages
 * https://dmoj.ca/problem/ecoo21p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the problem data. */
    for _ in 0..3 {
        std::io::stdin().read_line(&mut buffer).unwrap();
    }

    /* Parse the probem data. */
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* What time is the message read. */
    println!(
        "{}",
        if let Some(time) = message_read_time(data[0], data[1], data[2]) {
            format!("{}", time)
        } else {
            "Who knows...".to_string()
        }
    );
}

/// Determine what time the message is read if at all.
fn message_read_time(home_start: usize, check_iterval: usize, msg_sent: usize) -> Option<usize> {
    let num_checks = 3;

    /* Each time a check is performed determined if the message was read. */
    for chck_idx in 1..=num_checks {
        let check_time = home_start + check_iterval * chck_idx;

        if check_time >= msg_sent {
            return Some(check_time);
        }
    }

    return None;
}
