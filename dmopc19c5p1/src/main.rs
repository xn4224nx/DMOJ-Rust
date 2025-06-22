/*
 * DMOPC '19 Contest 5 P1 - Conspicuous Cryptic Checklist
 * https://dmoj.ca/problem/dmopc19c5p1
 */

use std::collections::HashSet;

fn main() {
    let mut comp_assignments = 0;
    let mut buffer = String::new();

    /* Read the number of items in the bag and the number of assignments. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    /* Read the bag items. */
    let mut tzak_bag: HashSet<String> = HashSet::new();
    for _ in 0..data[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        tzak_bag.insert(buffer.clone());
    }

    /* Read the assignments. */
    for _ in 0..data[1] {
        let mut all_items_in_bag = true;

        /* How many items are in the assingment */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let num_vals_in_asg = buffer.trim().parse::<usize>().unwrap();

        /* Read the items and check they are in the bag. */
        for _ in 0..num_vals_in_asg {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();

            if !tzak_bag.contains(&buffer) {
                all_items_in_bag = false;
            }
        }

        /* Can this assignment be completed? */
        if all_items_in_bag {
            comp_assignments += 1;
        }
    }
    println!("{}", comp_assignments);
}
