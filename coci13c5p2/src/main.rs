/*
 * COCI '13 Contest 5 #2 Obilazak
 * https://dmoj.ca/problem/coci13c5p2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of levels of the binary tree. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_levels = buffer.trim().parse::<usize>().unwrap();

    /* Read the elements of the tree. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let elements = buffer
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    /* Print the binary tree top to bottom, each node will have two children
     * except the last layer, which will have none. */
    for level in 0..num_levels {
        let mut level_out = String::new();

        /* There will be 2^level nodes per level. */
        let num_nodes = 2usize.pow(level as u32);
        let splits = 2 * num_nodes;

        for n_split in 0..num_nodes {
            let ele_idx = (1 + 2 * n_split) * elements.len() / splits;
            level_out.push_str(&format!("{} ", elements[ele_idx]));
        }
        level_out.pop();
        println!("{}", level_out);
    }
}
