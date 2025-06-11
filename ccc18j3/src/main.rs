/*
 * CCC '18 J3 - Are we there yet?
 * https://dmoj.ca/problem/ccc18j3
 */

fn main() {
    let mut buffer = String::new();

    /* Read the distances between the cities. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dists: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let num_cities = dists.len() + 1;
    let mut all_dists = vec![vec![0; num_cities]; num_cities];

    /* Calculate the city distances */
    for cit_0 in 0..num_cities {
        for cit_1 in cit_0..num_cities {
            all_dists[cit_0][cit_1] = dists[cit_0..cit_1].iter().sum();
            all_dists[cit_1][cit_0] = all_dists[cit_0][cit_1];
        }
    }

    /* Print the distance matrix row by row. */
    for line_idx in 0..num_cities {
        println!(
            "{}",
            all_dists[line_idx]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
