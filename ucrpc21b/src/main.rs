/*
 * UCRPC F21 B - Lost in the Shuffle
 * https://dmoj.ca/problem/ucrpc21b
 */

const START_DOLL: usize = 3;

fn main() {
    let mut buffer = String::new();
    let mut doll = START_DOLL;

    /* How many switches are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_switches = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the switches. */
    for _ in 0..num_switches {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let switch = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Only execute switches the doll is involved in. */
        if switch[0] == doll {
            doll = switch[1];
        } else if switch[1] == doll {
            doll = switch[0];
        }
    }

    /* After all the swaps where is the doll? */
    println!("{}", doll);
}
