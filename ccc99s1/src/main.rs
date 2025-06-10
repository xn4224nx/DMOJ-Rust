/*
 * CCC '99 S1 - Card Game
 * https://dmoj.ca/problem/ccc99s1
 */

fn main() {
    let all_cards = read_cards();

    /* Keep track of the player characteristics. */
    let players = vec!['A', 'B'];
    let mut player_scores = vec![0; players.len()];
    let mut curr_player = 0;

    /* Turn over cards one by one and play the game*/
    for cidx in 0..52 {
        /* Check if this card scores */
        'score_check: for scr_idx in 0..4 {
            let points = 4 - scr_idx;

            /* Ensure no face cards are up and coming */
            if all_cards[cidx] == 12 - scr_idx && cidx < 52 - points {
                for next_id in 1..=points {
                    if all_cards[cidx + next_id] > 8 {
                        continue 'score_check;
                    }
                }
                println!(
                    "Player {} scores {} point(s).",
                    players[curr_player], points
                );
                player_scores[curr_player] += points;
            }
        }

        /* Swap to the next player. */
        curr_player = (curr_player + 1) % players.len();
    }

    for pl_idx in 0..players.len() {
        println!(
            "Player {}: {} point(s).",
            players[pl_idx], player_scores[pl_idx]
        );
    }
}

fn read_cards() -> Vec<usize> {
    let mut buffer = String::new();
    let mut all_cards = Vec::with_capacity(52);

    /* Read the cards line by line. */
    for _ in 0..52 {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Determine what the card is */
        all_cards.push(match buffer.as_str() {
            "two\n" => 0,
            "three\n" => 1,
            "four\n" => 2,
            "five\n" => 3,
            "six\n" => 4,
            "seven\n" => 5,
            "eight\n" => 6,
            "nine\n" => 7,
            "ten\n" => 8,
            "jack\n" => 9,
            "queen\n" => 10,
            "king\n" => 11,
            "ace\n" => 12,
            _ => panic!("Unrecognised line: '{}'", buffer),
        });
    }
    return all_cards;
}
