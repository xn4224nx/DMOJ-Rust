/*
 * IOI '14 Practice Task 2 - Station
 * https://dmoj.ca/problem/ioi14pp2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of stations and travel limit. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let jour_info: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Unable to parse traval info"))
        .collect();

    /* Determine which stations have lodges. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let lodge_locs: Vec<bool> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Unable to parse station value") != 0)
        .collect();

    println!("{}", min_days(&lodge_locs, jour_info[1]));
}

/// Determine the minimum days required to travel from the first to last station
fn min_days(stations: &Vec<bool>, max_move_dist: usize) -> i32 {
    let mut next_station = 0;
    let mut curr_station = 0;
    let mut num_days = 0;

    while curr_station < stations.len() - 1 {
        num_days += 1;

        /* Find the next possible station that is furthest away. */
        for mv in (0..=max_move_dist).rev() {
            let station_idx = curr_station + mv;

            /* Ensure the station exists. */
            if station_idx >= stations.len() {
                continue;
            }

            /* See if the station has a travel lodge. */
            if stations[curr_station + mv] {
                next_station = curr_station + mv;
                break;
            }
        }

        /* If no move is possible return a fail code */
        if next_station == curr_station {
            return -1;
        }
        curr_station = next_station
    }
    return num_days;
}
