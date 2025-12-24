/*
 * DWITE '08 R4 #1 - Four player Pong with no players
 * https://dmoj.ca/problem/dwite08c4p1
 */

const NUM_GAMES: usize = 5;
const START: (i32, i32) = (50, 25);
const SCREEN_SIZE: (i32, i32) = (100, 50);

fn main() {
    let mut buffer = String::new();

    /* Play the games. */
    for _ in 0..NUM_GAMES {
        buffer.clear();

        /* Read the x velocity. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let x_vel = buffer.trim_end().parse::<i32>().unwrap();
        buffer.clear();

        /* Read the y velocity. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let y_vel = buffer.trim_end().parse::<i32>().unwrap();

        /* The ball starts in the center of the screen. */
        let (mut x_pos, mut y_pos) = START;

        /* Calculate where the ball will hit the edge of the screen. */
        while 0 < x_pos && x_pos < SCREEN_SIZE.0 && 0 < y_pos && y_pos < SCREEN_SIZE.1 {
            x_pos += x_vel;
            y_pos += y_vel;
        }
        println!("{},{}", x_pos, y_pos);
    }
}
