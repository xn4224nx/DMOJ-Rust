/*
 * CCC '16 J4 - Arrival Time
 * https://dmoj.ca/problem/ccc16j4
 */

const DIST_TO_WORK: usize = 60 * 4;

fn main() {
    let rush_hours = vec![(7 * 60, 10 * 60), (15 * 60, 19 * 60)];
    let mut buffer = String::new();
    let mut dist_traveled = 0;

    /* Read the departure time. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let raw_dep_time = buffer
        .trim_end()
        .split(":")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut curr_time = raw_dep_time[0] * 60 + raw_dep_time[1];

    /* Travel, monitoring the distance and time elapsed. */
    'stepping: while dist_traveled < DIST_TO_WORK {
        curr_time %= 60 * 24;

        /* If within a rush hour only move one. */
        for rsh_idx in 0..rush_hours.len() {
            if curr_time >= rush_hours[rsh_idx].0 && curr_time < rush_hours[rsh_idx].1 {
                curr_time += 1;
                dist_traveled += 1;
                continue 'stepping;
            }
        }

        /* If outside a rush hour. */
        curr_time += 1;
        dist_traveled += 2;
    }

    /* Correct the display of midnight. */
    if curr_time == 60 * 24 {
        curr_time = 0;
    }

    /* Show the arrival time. */
    println!("{:02}:{:02}", curr_time / 60, curr_time % 60);
}
