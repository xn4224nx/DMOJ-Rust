/*
 * CCC '21 J3 - Secret Instructions
 * https://dmoj.ca/problem/ccc21j3
 */

const EXIT_VALUE: usize = 99999;

fn main() {
    let mut buffer = String::new();
    let mut prev_dir = 0;

    /* Read instructions until given the exit value. */
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let direction = buffer.trim().parse::<usize>().unwrap();

        /* Destination reached. */
        if direction == EXIT_VALUE {
            break;
        }

        /* Parse the three values. */
        let dir0 = direction / 10000;
        let dir1 = (direction / 1000) % 10;
        let dist = direction % 1000;

        let mut curr_dir = dir0 + dir1;

        /* Use the previous direction. */
        if curr_dir == 0 {
            curr_dir = prev_dir;
        }

        /* Print the human readable instruction. */
        println!(
            "{} {}",
            if curr_dir % 2 == 0 { "right" } else { "left" },
            dist
        );
        prev_dir = curr_dir;
    }
}
