/*
 * SAC '21 Code Challenge P2 - Littering
 * https://dmoj.ca/problem/sac21ccp2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the ammount of garbage and what you can pick up. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_garb = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Read the dirtiness of the all rubbish. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut all_rubbish = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Sort the rubbish largest to smallest. */
    all_rubbish.sort();
    all_rubbish.reverse();

    /* What is the most dirtiness that can be removed? */
    println!("{}", all_rubbish.iter().take(num_garb[1]).sum::<usize>());
}
