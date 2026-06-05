/*
 * UCRPC F21 A - Senseless Census
 * https://dmoj.ca/problem/ucrpc21a
 */

const CENSUS_ELE: char = 't';

fn main() {
    let mut buffer = String::new();
    let mut ele_count = 0;

    /* Read the problem metadata */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let meta = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Count instances of the key character. */
    for _ in 0..meta[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        for ele in buffer.chars() {
            if ele == CENSUS_ELE {
                ele_count += 1;
            }
        }
    }
    println!("{}", ele_count);
}
