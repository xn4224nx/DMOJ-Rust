/*
 * CCC '11 J4 - Boring Business
 * https://dmoj.ca/problem/ccc11j4
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    let mut curr_c = (-1, -5);
    let mut tunnels = HashSet::from([
        (0, -1),
        (0, -2),
        (0, -3),
        (1, -3),
        (2, -3),
        (3, -3),
        (3, -4),
        (3, -5),
        (4, -5),
        (5, -5),
        (5, -4),
        (5, -3),
        (6, -3),
        (7, -3),
        (7, -4),
        (7, -5),
        (7, -6),
        (7, -7),
        (6, -7),
        (5, -7),
        (4, -7),
        (3, -7),
        (2, -7),
        (1, -7),
        (0, -7),
        (-1, -7),
        (-1, -6),
        (-1, -5),
    ]);

    /* Read input until the quit command or a dangerous intersection. */
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let data = buffer
            .trim_end()
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        /* What is the command for this line? */
        let cmd = data[0].chars().next().unwrap();

        /* Has the exit command been given? */
        if cmd == 'q' {
            break;
        }

        /* Has this move brought danger? */
        let mut danger = false;

        /* How far in the direction are they moving? */
        let move_mag = data[1].parse::<i32>().unwrap();

        /* Move in that direction and check that there are no colisions. */
        for _ in 0..move_mag {
            match cmd {
                'l' => {
                    curr_c = (curr_c.0 - 1, curr_c.1);
                }
                'r' => {
                    curr_c = (curr_c.0 + 1, curr_c.1);
                }
                'u' => {
                    curr_c = (curr_c.0, curr_c.1 + 1);
                }
                'd' => {
                    curr_c = (curr_c.0, curr_c.1 - 1);
                }
                _ => panic!("Unsuported direction!"),
            };

            /* Is this location part of another tunnel? */
            if !danger {
                if !tunnels.insert(curr_c.clone()) {
                    danger = true;
                }
            };
        }

        /* Show the result of the move. */
        println!(
            "{} {} {}",
            curr_c.0,
            curr_c.1,
            if danger { "DANGER" } else { "safe" }
        );

        /* Stop moving after any dangerous move. */
        if danger {
            break;
        }
    }
}
