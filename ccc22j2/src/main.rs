/*
 * CCC '22 J2 - Fergusonball Ratings
 * https://dmoj.ca/problem/ccc22j2
 */

const SCORE_THRESHOLD: usize = 40;
const PNT_SCORE: usize = 5;
const FOUL_SCORE: usize = 3;

fn main() {
    let mut players_gt_thresh = 0;
    let mut buffer = String::new();

    /* Read the number of players. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_players = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the player points and fouls */
    for _ in 0..num_players {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let points = buffer.trim_end().parse::<usize>().unwrap();
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let fouls = buffer.trim_end().parse::<usize>().unwrap();

        /* Calculate the rating of the player. */
        let ply_rating = (points * PNT_SCORE).saturating_sub(fouls * FOUL_SCORE);

        /* Record the count of the players greater than the threshold.  */
        if ply_rating > SCORE_THRESHOLD {
            players_gt_thresh += 1;
        }
    }

    /* Show the number of players greater than the threshold. */
    println!(
        "{}{}",
        players_gt_thresh,
        if players_gt_thresh == num_players {
            "+"
        } else {
            ""
        }
    );
}
