/*
 * ECOO '17 R1 P1 - Munch 'n' Brunch
 * https://dmoj.ca/problem/ecoo17r1p1
 */

fn main() {
    let year_prices = vec![12.0, 10.0, 7.0, 5.0];
    let mut buffer = String::new();

    /* Calculate the viability of each 10 trips */
    for _ in 0..10 {
        buffer.clear();

        /* Read the trip cost */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let trip_cost = buffer.trim().parse::<f32>().unwrap();

        /* Read the year percentages and find the index of the biggest one. */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mut year_percs: Vec<f32> = Vec::new();
        let mut largest_perc = 0.0;
        let mut large_perc_idx = 0;
        for (p_idx, perc) in buffer.trim().splitn(year_prices.len(), ' ').enumerate() {
            let perc_val = perc.parse::<f32>().unwrap();
            year_percs.push(perc_val);

            /* Is this percentage the current highest. */
            if largest_perc < perc_val {
                largest_perc = perc_val;
                large_perc_idx = p_idx;
            }
        }

        /* Read the total student population at the brunch */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let total_students = buffer.trim().parse::<u32>().unwrap();

        /* Calculate the number of students in each year. */
        let mut student_nums: Vec<u32> = (0..year_prices.len())
            .map(|x| (year_percs[x] * total_students as f32).floor() as u32)
            .collect();

        /* Calculate the difference in the number of students. */
        let student_diff = total_students - student_nums.iter().sum::<u32>();

        /* Change the largest student group so the totals match. */
        student_nums[large_perc_idx] += student_diff;
        assert_eq!(total_students, student_nums.iter().sum::<u32>());

        /* Calculate the money raised at the brunch. */
        let raised_funds = 0.5
            * (0..year_prices.len())
                .map(|x| student_nums[x] as f32 * year_prices[x])
                .sum::<f32>();

        /* Does additional funding need to be found? */
        println!(
            "{}",
            if raised_funds < trip_cost {
                "YES"
            } else {
                "NO"
            }
        );
    }
}
