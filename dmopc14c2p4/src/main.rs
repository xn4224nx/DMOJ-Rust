/*
 * DMOPC '14 Contest 2 P4 - Deforestation
 * https://dmoj.ca/problem/dmopc14c2p4
 */

fn main() {
    let mut buffer = String::new();

    /* How many trees are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_trees = buffer.trim_end().parse::<usize>().unwrap();

    let mut tree_prefx_sum = Vec::with_capacity(num_trees + 1);
    tree_prefx_sum.push(0);

    /* Read the tree sizes. */
    for _ in 0..num_trees {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let tree_height = buffer.trim_end().parse::<usize>().unwrap();

        /* Create a prefix sum array of tree sizes */
        tree_prefx_sum.push(tree_height + tree_prefx_sum.last().unwrap())
    }

    /* How many queries are there? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_queries = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the queries. */
    for _ in 0..num_queries {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let tree_idxs = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Show the total wood gained by chopping down all the tree between these two. */
        println!(
            "{}",
            tree_prefx_sum[tree_idxs[1] + 1] - tree_prefx_sum[tree_idxs[0]]
        )
    }
}
