/*
 * Amplitude Hackathon Summer '25 Problem 3 - Jenny the Beli Influencer
 * https://dmoj.ca/problem/ampl2025sp3
 */

fn main() {
    let num_ratings = 3;
    let num_reviewers = 2;

    let mut buffer = String::new();

    /* Read the total number of restaruants */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_rest = buffer.trim_end().parse::<usize>().unwrap();

    /* Store the what review each resturant got as a number. */
    let mut reviews: Vec<Vec<usize>> = vec![vec![0; num_rest]; num_reviewers];

    /* Read all the reviewer's ratings. */
    for review_idx in 0..num_reviewers {
        for rate_idx in 0..num_ratings {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();

            /*  Top rated restaruants are alreadly labeled as default. */
            if rate_idx == 0 {
                continue;
            }

            for rest_num in buffer.trim_end().split(' ').skip(1) {
                let rest_idx = rest_num.parse::<usize>().unwrap() - 1;
                reviews[review_idx][rest_idx] = rate_idx;
            }
        }
    }

    /* Count the number of restaraunts that all the reviewers agree on. */
    println!(
        "{}",
        (0..num_rest)
            .filter(|x| reviews[0][*x] == reviews[1][*x])
            .count()
    );
}
