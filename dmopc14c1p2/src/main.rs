/*
 * DMOPC '14 Contest 1 P2 - Tiles
 * https://dmoj.ca/problem/dmopc14c1p2
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse the three problem values. */
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* How many square  tiles could fit in the room? */
    println!("{}", (data[0] / data[2]) * (data[1] / data[2]));
}
