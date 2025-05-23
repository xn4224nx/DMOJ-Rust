/*
 * IOI '04 P5 - Farmer
 * https://dmoj.ca/problem/ioi04p5
 */

use itertools::Itertools;

fn main() {
    let mut max_olives = 0;

    /* Read the specifics of the problem. */
    let tree_stats = read_nums(3);
    let tree_capacity = tree_stats[0];

    /* Read the cypress tree field sizes */
    let fields = read_nums(tree_stats[1]);

    /* Read the cypress tree strip sizes */
    let strips = read_nums(tree_stats[2]);

    /* Create a combines lookup of cypress tree amounts and olive tree amounts. */
    let cypress_trees = [fields.clone(), strips.clone()].concat();
    let olive_trees = [fields, strips.into_iter().map(|x| x - 1).collect()].concat();

    /* Iterate over every combination of strips and fields */
    for n_items in 0..cypress_trees.len() {
        for tree_comb in (0..cypress_trees.len()).combinations(n_items) {
            /* Only consider combinations that are at the tree capacity */
            if tree_comb.iter().map(|x| cypress_trees[*x]).sum::<u32>() != tree_capacity {
                break;
            }

            /* Calculate the olive trees this combination gives. */
            let num_olives = tree_comb.iter().map(|x| olive_trees[*x]).sum::<u32>();

            /* Has a new maximum number of olives been found? */
            if max_olives < num_olives {
                max_olives = num_olives
            }
        }
    }
    println!("{}", max_olives);
}

fn read_nums(num_count: u32) -> Vec<u32> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer
        .splitn(num_count as usize, ' ')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();
}
