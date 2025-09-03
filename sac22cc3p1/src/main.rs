/*
 * SAC '22 Code Challenge 3 P1 - Hair Hazards
 * https://dmoj.ca/problem/sac22cc3p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the initial hair length. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let inti_hair_len = buffer.trim().parse::<usize>().unwrap();

    /* Read the amount of hair removed after each cut. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let cut_mag = buffer.trim().parse::<usize>().unwrap();

    /* Read the total number of hair cuts. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cuts = buffer.trim().parse::<usize>().unwrap();

    /* Print the hair length after each cut. */
    for cut_idx in 1..=num_cuts {
        println!("{}", inti_hair_len - cut_idx * cut_mag);
    }
}
