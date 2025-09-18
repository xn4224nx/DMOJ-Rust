/*
 * ACM U of T Tryouts C0 B - Interleaving Leaves
 * https://dmoj.ca/problem/acmtryouts0b
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of scenario. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let scenarios = buffer.trim_end().parse::<usize>().unwrap();

    for _ in 0..scenarios {
        buffer.clear();

        /* How many leaves are in each pile. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let num_leaves = buffer.trim_end().parse::<usize>().unwrap();

        /* Read the two piles of leaves. */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let pile_a = buffer.trim_end().chars().collect::<Vec<char>>();
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let pile_b = buffer.trim_end().chars().collect::<Vec<char>>();

        /* Interlace the two piles of leaves. */
        for leaf_idx in (0..num_leaves).rev() {
            print!("{}{}", pile_b[leaf_idx], pile_a[leaf_idx]);
        }
        println!();
    }
}
