/*
 * COCI '17 Contest 2 #1 Košnja
 * https://dmoj.ca/problem/coci17c2p1
 */

fn main() {
    let mut buffer = String::new();

    /* How many pieces of land are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_lands = buffer.trim_end().parse::<usize>().unwrap();

    /* Read and solve the minimum operations needed. */
    for _ in 0..num_lands {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let land_dims = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Determine the minimum number of turns needed to mow the land. */
        println!("{}", 2 * std::cmp::min(land_dims[0] - 1, land_dims[1] - 1));
    }
}
