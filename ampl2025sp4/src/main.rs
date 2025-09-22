/*
 * Amplitude Hackathon Summer '25 Problem 4 - Regina the Social Media Manager
 * https://dmoj.ca/problem/ampl2025sp4
 */

use std::collections::{HashMap, HashSet};

fn main() {
    let mut buffer = String::new();

    /* How many users are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_users = buffer.trim_end().parse::<usize>().unwrap();

    /* Create lookups for social and devices to user. */
    let mut device: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut social: HashMap<usize, HashSet<usize>> = HashMap::new();

    /* Read the user details */
    for user_idx in 0..num_users {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let udata = buffer
            .trim_end()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Save the device data */
        device
            .entry(udata[0])
            .and_modify(|x| {
                x.insert(user_idx);
            })
            .or_insert(HashSet::from([user_idx]));

        /* Save the social media data. */
        social
            .entry(udata[1])
            .and_modify(|x| {
                x.insert(user_idx);
            })
            .or_insert(HashSet::from([user_idx]));
    }

    /* How many queries are there? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_queries = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the queries and count the users they apply to. */
    for _ in 0..num_queries {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let query = buffer
            .trim_end()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Determine of there are matching devices or social for this query. */
        let device_exists = device.contains_key(&query[0]);
        let social_exists = social.contains_key(&query[1]);

        let num_users = if device_exists && social_exists {
            device[&query[0]]
                .union(&social[&query[1]])
                .collect::<HashSet<&usize>>()
                .len()
        } else if device_exists && !social_exists {
            device[&query[0]].len()
        } else if !device_exists && social_exists {
            social[&query[1]].len()
        } else {
            0
        };

        println!("{}", num_users);
    }
}
