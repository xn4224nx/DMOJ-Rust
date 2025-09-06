/*
 * CCC '14 J3 - Double Dice
 * https://dmoj.ca/problem/ccc14j3
 */

fn main() {
    let mut buffer = String::new();
    let mut scores: Vec<usize> = vec![100; 2];

    /* Read the number of games. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_games = buffer
        .trim_end()
        .parse::<usize>()
        .expect("The number of games couldn't be parsed!");

    /* Read the dice rolls for the games and determine who wins the game. */
    for _ in 0..num_games {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let rolls = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().expect("Dice roll could not be parsed"))
            .collect::<Vec<usize>>();

        /* The first player wins. */
        if rolls[0] > rolls[1] {
            scores[1] -= rolls[0];

        /* The second player wins. */
        } else if rolls[0] < rolls[1] {
            scores[0] -= rolls[1];

        /* A draw means the points are unchanged. */
        } else {
            continue;
        }
    }
    println!("{}\n{}", scores[0], scores[1]);
}
