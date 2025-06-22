/*
 * DMOPC '19 Contest 5 P2 - Charlie's Crazy Conquest
 * https://dmoj.ca/problem/dmopc19c5p2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of moves and starting health. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    /* Define the variables for the battle simulation. */
    let mut game_moves: Vec<(char, usize)> = vec![('A', 0); data[0] * 2];
    let mut health_vals: Vec<usize> = vec![data[1]; 2];
    let mut current_player = 0;
    let mut other_player = 1;

    /* Read Charlie's moves. */
    for mv_idx in 0..data[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let (move_g, val) = buffer.trim().split_once(" ").unwrap();
        game_moves[mv_idx * 2] = (move_g.chars().next().unwrap(), val.parse().unwrap());
    }

    /* Read the Bot's moves. */
    for mv_idx in 0..data[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let (move_g, val) = buffer.trim().split_once(" ").unwrap();
        game_moves[mv_idx * 2 + 1] = (move_g.chars().next().unwrap(), val.parse().unwrap());
    }

    /* Simulate the battle. */
    for mv_idx in 0..game_moves.len() {
        /* The current player attacks. */
        if game_moves[mv_idx].0 == 'A' && (mv_idx == 0 || game_moves[mv_idx - 1].0 != 'D') {
            health_vals[other_player] =
                health_vals[other_player].saturating_sub(game_moves[mv_idx].1);

        /* The current player dodges. */
        } else if mv_idx >= game_moves.len() - 1 || game_moves[mv_idx + 1].0 == 'D' {
            health_vals[current_player] =
                health_vals[current_player].saturating_sub(game_moves[mv_idx].1);
        };

        /* Check to see if charlie is dead */
        if health_vals[0] == 0 {
            println!("DEFEAT");
            return;
        }

        /* Check to see if the bot is dead */
        if health_vals[1] == 0 {
            println!("VICTORY");
            return;
        }

        /* Swap Players */
        (current_player, other_player) = (other_player, current_player);
    }
    println!("TIE");
}
