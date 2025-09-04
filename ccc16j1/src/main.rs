/*
 * CCC '16 J1 - Tournament Selection
 * https://dmoj.ca/problem/ccc16j1
 */

fn main() {
    let mut buffer = String::new();
    let mut player_wins = 0;

    /* Read the player game results. */
    for _ in 0..6 {
        std::io::stdin().read_line(&mut buffer).unwrap();
        player_wins += (buffer == "W\n") as usize;
        buffer.clear();
    }

    /* Print the group the player is in. */
    println!(
        "{}",
        match player_wins {
            0 => "-1",
            1 | 2 => "3",
            3 | 4 => "2",
            5 | 6 => "1",
            _ => panic!("Player win ammount {} is not accounted for.", player_wins),
        }
    );
}
