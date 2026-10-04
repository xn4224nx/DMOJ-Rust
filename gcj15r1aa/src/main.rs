/*
 * Google Code Jam '15 Round 1A Problem A - Mushroom Monster
 * https://dmoj.ca/problem/gcj15r1aa
 */

fn main() {
    let mut buffer = String::new();

    /* How many cases are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cases = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the case data. */
    for case_idx in 1..=num_cases {
        buffer.clear();

        /* Ignore the first line that has the number of mushrooms. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.clear();

        /* Read the number of mushroom pieces on the plate at 10s intervals. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mushroom_pieces = buffer
            .trim_end()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        /* Calculate the minimum number the diner could have eaten. */
        let (num_eaten_1, num_eaten_2) = min_num_mush_eaten(&mushroom_pieces);
        println!("Case #{}: {} {}", case_idx, num_eaten_1, num_eaten_2,);
    }
}

/// For the first method assume the diner could eat any number of mushroom
/// pieces at any time. For the second method assume that, starting with the
/// first time we look at the plate, the diner eats mushrooms at a constant rate
/// whenever there are mushrooms on their plate.
fn min_num_mush_eaten(mush_pieces: &Vec<usize>) -> (usize, usize) {
    let mut min_eaten = (0, 0);
    let mut max_consumed_in_iterval = 0;

    /* Find the drops in the number of mushrooms. */
    for idx in 1..mush_pieces.len() {
        let drop_in_mushrooms = mush_pieces[idx - 1].saturating_sub(mush_pieces[idx]);

        /* For the first method sum the drops in mushrooms. */
        min_eaten.0 += drop_in_mushrooms;

        /* Find the maximum number of mushrooms eaten in an interval.*/
        if drop_in_mushrooms > max_consumed_in_iterval {
            max_consumed_in_iterval = drop_in_mushrooms;
        }
    }

    /* Given the max consumed in an interval deterimine how many where eaten. */
    if max_consumed_in_iterval > 0 {
        for idx in 0..(mush_pieces.len() - 1) {
            min_eaten.1 += std::cmp::min(max_consumed_in_iterval, mush_pieces[idx]);
        }
    }
    return min_eaten;
}
