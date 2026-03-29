/*
 * DMOPC '17 Contest 2 P1 - 0-1 Knapsack
 * https://dmoj.ca/problem/dmopc17c2p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of items. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_items = buffer.trim_end().parse::<usize>().unwrap();

    let mut weights = Vec::with_capacity(num_items);

    /* Read the items and their details. */
    for _ in 0..num_items {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Ignore negative numbers as these are items we don't want. */
        let itm_data = buffer
            .trim_end()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        /* Only consider items with positive value */
        if itm_data.len() == 2 && itm_data[1] > 0 {
            weights.push(itm_data[0]);
        }
    }

    /* Get a knapscak that can fit all the items with a positive value. */
    println!("{}", weights.into_iter().sum::<usize>());
}
