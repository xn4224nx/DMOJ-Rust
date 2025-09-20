/*
 * COCI '06 Contest 2 #4 Sjecišta
 * https://dmoj.ca/problem/coci06c2p4
 */

fn main() {
    let mut buffer = String::new();

    /* How many sides does the shape have? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let sides = buffer.trim_end().parse::<usize>().unwrap();

    /* Output the number of intersections on all the lines. */
    println!("{}", sides * (sides - 1) * (sides - 2) * (sides - 3) / 24);
}
