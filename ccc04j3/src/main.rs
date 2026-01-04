/*
 * CCC '04 J3 - Smile with Similes
 * https://dmoj.ca/problem/ccc04j3
 */

fn main() {
    let mut buffer = String::new();

    /* How many adjectives will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_adjcs = buffer.trim_end().parse::<usize>().unwrap();

    /* How many nouns will there be? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_nouns = buffer.trim_end().parse::<usize>().unwrap();

    let mut adjcs = Vec::with_capacity(num_adjcs);
    let mut nouns = Vec::with_capacity(num_nouns);

    /* Read the adjectives. */
    for _ in 0..num_adjcs {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();
        adjcs.push(buffer.clone());
    }

    /* Read the nouns. */
    for _ in 0..num_nouns {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();
        nouns.push(buffer.clone());
    }

    /* Print all similes combinations. */
    for adj_idx in 0..num_adjcs {
        for noun_idx in 0..num_nouns {
            println!("{} as {}", adjcs[adj_idx], nouns[noun_idx]);
        }
    }
}
