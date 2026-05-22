/*
 * COCI '15 Contest 6 #1 Bela
 * https://dmoj.ca/problem/coci15c6p1
 */

const HAND_SIZE: usize = 4;

fn main() {
    let mut buffer = String::new();
    let mut total_value = 0;

    /* Read the number of hands and the dominant suit. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let (rw_num_hands, rw_dom_suit) = buffer.trim_end().split_once(" ").unwrap();

    /* Parse the problem data. */
    let num_hands = rw_num_hands.parse::<usize>().unwrap();
    let dom_suit = rw_dom_suit.chars().next().unwrap();

    /* Read the hand data. */
    for _ in 0..(HAND_SIZE * num_hands) {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Parse the hand data. */
        let mut hand_stream = buffer.chars();
        let card = hand_stream.next().unwrap();
        let suit = hand_stream.next().unwrap();

        /* Calculate the hand value. */
        total_value += if suit == dom_suit {
            match card {
                'A' => 11,
                'K' => 4,
                'Q' => 3,
                'J' => 20,
                'T' => 10,
                '9' => 14,
                _ => 0,
            }
        } else {
            match card {
                'A' => 11,
                'K' => 4,
                'Q' => 3,
                'J' => 2,
                'T' => 10,
                _ => 0,
            }
        };
    }

    println!("{}", total_value);
}
