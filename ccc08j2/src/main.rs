/*
 * CCC '08 J2 - Do the Shuffle
 * https://dmoj.ca/problem/ccc08j2
 */

use std::collections::VecDeque;

fn main() {
    let mut playlist = VecDeque::from(['A', 'B', 'C', 'D', 'E']);

    /* Every pair of buttons move the playlist around. */
    loop {
        let (button, repeats) = read_nums();

        /* Change the playlist */
        match button {
            1 => playlist.rotate_left(repeats as usize),
            2 => playlist.rotate_right(repeats as usize),
            3 => {
                let tmp0 = playlist[0];
                let tmp1 = playlist[1];
                playlist[1] = tmp0;
                playlist[0] = tmp1;
            }
            4 => break,
            _ => panic!("{} is not a recognised button!", button),
        }
    }

    /* Print the final playlist. */
    for (idx, song) in playlist.iter().enumerate() {
        print!("{}", song);

        if idx == playlist.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}

fn read_nums() -> (u32, u32) {
    let mut buffer = String::new();

    /* Read the first number. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num0 = buffer.trim().parse::<u32>().unwrap();
    buffer.clear();

    /* Read the second number. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num1 = buffer.trim().parse::<u32>().unwrap();

    return (num0, num1);
}
