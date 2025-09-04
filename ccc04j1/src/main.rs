/*
 * CCC '04 J1 - Squares
 * https://dmoj.ca/problem/ccc04j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of tiles. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_tiles = buffer.trim_end().parse::<usize>().unwrap();

    println!("The largest square has side length {}.", num_tiles.isqrt())
}
