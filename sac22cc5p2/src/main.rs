/*
 * SAC '22 Code Challenge 5 P2 - Querying Extensions
 * https://dmoj.ca/problem/sac22cc5p2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of extentions. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_extens = buffer.trim().parse::<usize>().unwrap();

    /* Read the intial extention sway. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let on_left = buffer == String::from("left\n");

    /* If the number of extentions is even the sway will swap. */
    if num_extens % 2 == 0 && on_left {
        println!("right");
    } else if num_extens % 2 != 0 && on_left {
        println!("left");
    } else if num_extens % 2 == 0 && !on_left {
        println!("left");
    } else {
        println!("right");
    }
}
