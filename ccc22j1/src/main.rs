/*
 * CCC '22 J1 - Cupcake Party
 * https://dmoj.ca/problem/ccc22j1
 */

const NUM_IN_CLASS: usize = 28;
const REG_BOX_SIZE: usize = 8;
const SML_BOX_SIZE: usize = 3;

fn main() {
    let mut buffer = String::new();

    /* Read the number of regular boxes. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_regul = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the number of small boxes. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_small = buffer.trim_end().parse::<usize>().unwrap();

    /* How many cakes are left over? */
    println!(
        "{}",
        (num_regul * REG_BOX_SIZE + num_small * SML_BOX_SIZE).saturating_sub(NUM_IN_CLASS)
    );
}
