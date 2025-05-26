/*
 * COCI '17 Contest 1 #1 Cezar
 * https://dmoj.ca/problem/coci17c1p1
 */

fn main() {
    let mut viable_cards = 0;
    let (drawn_cards, hand_sum, hand_size) = read_cards();

    /* Calculate how much lower the value of the drawn cards is to 21. */
    let mut deficit = 19_usize.saturating_sub(hand_sum);

    /* Catch the issue of the hand dficite being larger than the largest card. */
    if deficit > 9 {
        deficit = 9;
    }

    /* Count the cards still in the deck that are equal or less to the deficit. */
    for card_idx in 0..=deficit {
        if card_idx == 8 {
            viable_cards += 16 - drawn_cards[card_idx]
        } else {
            viable_cards += 4 - drawn_cards[card_idx]
        }
    }

    /* If it likely that the next card drawn will be viable, draw one. */
    if 52 - hand_size - viable_cards < viable_cards {
        println!("VUCI");

    /* Otherwise stop! */
    } else {
        println!("DOSTA");
    }
}

/// From stdin read the indexes of the drawn cards and the sum of the hand and its size.
fn read_cards() -> (Vec<usize>, usize, usize) {
    let mut hand = vec![0; 10];
    let mut hand_sum = 0;
    let mut buffer = String::new();

    /* Determine the number of cards that will be read. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cards = buffer.trim().parse::<usize>().unwrap();

    /* Read the cards. */
    for _ in 0..num_cards {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let card = buffer.trim().parse::<usize>().unwrap();

        /* Sum the contents of the hand. */
        hand_sum += card;

        /* Record card by index */
        hand[card - 2] += 1;
    }
    return (hand, hand_sum, num_cards);
}
