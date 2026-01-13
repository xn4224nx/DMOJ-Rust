/*
 * CCC '03 S1 - Snakes and Ladders
 * https://dmoj.ca/problem/ccc03s1
 */

const MAX_SQR: usize = 100;
const QUIT_ROLL: usize = usize::MIN;

fn main() {
    let snakes_ladders = vec![(54, 19), (90, 48), (99, 77), (9, 34), (40, 64), (67, 86)];
    let mut buffer = String::new();
    let mut curr_sqr = 1;

    while curr_sqr != MAX_SQR {
        buffer.clear();

        /* Read the dice roll. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let dice = buffer.trim_end().parse::<usize>().unwrap();

        /* Detect the user giving up. */
        if dice == QUIT_ROLL {
            break;
        }

        /* Move the player. The player cannot go beyond the board. */
        if curr_sqr + dice <= MAX_SQR {
            curr_sqr += dice;
        }

        /* Determine if a ladder or snake has been stepped on. */
        for (s_sqr, f_sqr) in snakes_ladders.iter() {
            if curr_sqr == *s_sqr {
                curr_sqr = *f_sqr;
                break;
            }
        }

        /* Show the current player position. */
        println!("You are now on square {}", std::cmp::min(curr_sqr, MAX_SQR));
    }
    println!(
        "{}",
        if curr_sqr >= MAX_SQR {
            "You Win!"
        } else {
            "You Quit!"
        }
    );
}
