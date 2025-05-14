/*
 * https://dmoj.ca/problem/coci06c5p1
 */

use std::io;

fn main() {
    let mut input = String::new();
    let mut ball_pos = 1;

    /* Read the input. */
    io::stdin().read_line(&mut input).unwrap();

    /* Iterate over the commands. */
    for cmd in input.chars() {
        match cmd {
            'A' => {
                if ball_pos == 1 {
                    ball_pos = 2;
                } else if ball_pos == 2 {
                    ball_pos = 1;
                }
            }
            'B' => {
                if ball_pos == 2 {
                    ball_pos = 3;
                } else if ball_pos == 3 {
                    ball_pos = 2;
                }
            }
            'C' => {
                if ball_pos == 1 {
                    ball_pos = 3;
                } else if ball_pos == 3 {
                    ball_pos = 1;
                }
            }
            _ => continue,
        }
    }

    /* Return the final position of the ball. */
    println!("{}", ball_pos);
}
