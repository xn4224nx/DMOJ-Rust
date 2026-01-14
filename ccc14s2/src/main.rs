/*
 * CCC '14 S2 - Assigning Partners
 * https://dmoj.ca/problem/ccc14s2
 */

use std::collections::{HashMap, HashSet};

fn main() {
    let mut buffer = String::new();

    /* Ignore the first line. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Read the names. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let name_row_0 = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let name_row_1 = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    println!(
        "{}",
        if check_pairings(&name_row_0, &name_row_1) {
            "good"
        } else {
            "bad"
        }
    );
}

fn check_pairings(names_0: &Vec<String>, names_1: &Vec<String>) -> bool {
    assert_eq!(names_0.len(), names_1.len());

    /* Check no one is paired with themselves. */
    for n_idx in 0..names_0.len() {
        if names_0[n_idx] == names_1[n_idx] {
            return false;
        }
    }

    /* Check there are no duplicate names. */
    if names_0.iter().collect::<HashSet<&String>>().len() != names_0.len()
        || names_1.iter().collect::<HashSet<&String>>().len() != names_1.len()
    {
        return false;
    }

    /* Check that all pairings are consistent. */
    let mut pairings: HashMap<&String, &String> = HashMap::new();

    for n_idx in 0..names_0.len() {
        if pairings.contains_key(&names_0[n_idx])
            && **pairings.get(&names_0[n_idx]).unwrap() != names_1[n_idx]
        {
            return false;
        }
        pairings.insert(&names_1[n_idx], &names_0[n_idx]);
        pairings.insert(&names_0[n_idx], &names_1[n_idx]);
    }

    return true;
}
