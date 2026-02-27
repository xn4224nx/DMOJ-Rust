/*
 * Google Code Jam '14 Qualification Round Problem A - Magic Trick
 * https://dmoj.ca/problem/gcj14qra
 */

const NUM_CARD_DIMS: (usize, usize) = (4, 4);

fn main() {
    let mut buffer = String::new();

    /* How many cases will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cases = buffer.trim_end().parse::<usize>().unwrap();

    /* Solve each case in turn. */
    for case_idx in 1..=num_cases {
        println!(
            "Case #{}: {}",
            case_idx,
            determine_result(&read_possible_cards(), &read_possible_cards())
        );
    }
}

/// Read from STDIN the cards and volunteer choices.
fn read_possible_cards() -> Vec<bool> {
    let mut buffer = String::new();
    let mut cards_in_row = vec![false; NUM_CARD_DIMS.0 * NUM_CARD_DIMS.1];

    /* Read the row index the volunteer chose. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let vol_choi = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the rows of cards. */
    for rw_idx in 1..=NUM_CARD_DIMS.1 {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        if rw_idx == vol_choi {
            for card in buffer
                .trim_end()
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
            {
                cards_in_row[card - 1] = true;
            }
        }
    }
    return cards_in_row;
}

/// From the two rows detrmine what card the volenteer chose.
fn determine_result(cards_0: &Vec<bool>, cards_1: &Vec<bool>) -> String {
    let mut num_cards_the_same = 0;
    let mut last_cards_same = 0;

    assert_eq!(cards_0.len(), cards_1.len());

    /* Count the cards that are common to both rows. */
    for crd_idx in 0..cards_0.len() {
        if cards_0[crd_idx] && cards_1[crd_idx] {
            num_cards_the_same += 1;
            last_cards_same = crd_idx + 1;
        }
    }

    return if num_cards_the_same == 0 {
        String::from("Volunteer cheated!")
    } else if num_cards_the_same > 1 {
        String::from("Bad magician!")
    } else {
        last_cards_same.to_string()
    };
}
