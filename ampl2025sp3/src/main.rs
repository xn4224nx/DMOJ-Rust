/*
 * Amplitude Hackathon Summer '25 Problem 3 - Jenny the Beli Influencer
 * https://dmoj.ca/problem/ampl2025sp3
 */

fn main() {
    let num_ratings = 3;
    let mut buffer = String::new();

    /* Read the total number of restaruants */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_rest = buffer.trim_end().parse::<usize>().unwrap();

    /* Store the reviews as true for in that category */
    let mut reviews: Vec<Vec<bool>> = vec![vec![false; num_rest]; num_ratings * 2];

    /* Read all the reviewer's ratings. */
    for review_idx in 0..num_ratings * 2 {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        for rest_num in buffer.trim_end().split(' ').skip(1) {
            let rest_idx = rest_num.parse::<usize>().unwrap() - 1;
            reviews[review_idx][rest_idx] = true;
        }
    }

    /* Count many restaruant reviews match between the reviewers. */
    let mut num_matching_reviews = 0;
    for review_idx in 0..num_ratings {
        for rest_idx in 0..num_rest {
            if reviews[review_idx][rest_idx] == true
                && reviews[review_idx + num_ratings][rest_idx] == true
            {
                num_matching_reviews += 1
            }
        }
    }

    println!("{}", num_matching_reviews);
}
