/*
 * CCC '23 J1 - Deliv-e-droid
 * https://dmoj.ca/problem/ccc23j1
 */

const PACK_DELIV: i32 = 50;
const COLL_PENTY: i32 = 10;
const GREAT_BONS: i32 = 500;

fn main() {
    let mut buffer = String::new();

    /* How many packages are delivered and how many colisions happen? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("{}", game_score(data[0], data[1]));
}

fn game_score(num_pack: usize, num_colis: usize) -> i32 {
    let bonus = if num_pack > num_colis { GREAT_BONS } else { 0 };
    return (num_pack as i32) * PACK_DELIV - (num_colis as i32) * COLL_PENTY + bonus;
}
