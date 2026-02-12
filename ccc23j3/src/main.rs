/*
 * CCC '23 J3 - Special Event
 * https://dmoj.ca/problem/ccc23j3
 */

const NUM_DAYS: usize = 5;

fn main() {
    let mut buffer = String::new();
    let mut day_freq = vec![0; NUM_DAYS];

    /* Read the number of people. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_people = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the peoples availability. */
    for _ in 0..num_people {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        for (day, avl) in buffer.chars().enumerate() {
            if avl == 'Y' {
                day_freq[day] += 1;
            }
        }
    }

    /* Order the days by frequency of people who can attend. */
    let mut ord_freq = (0..NUM_DAYS)
        .into_iter()
        .zip(day_freq)
        .collect::<Vec<(usize, usize)>>();
    ord_freq.sort_by_key(|&(_, x)| x);
    ord_freq.reverse();

    /* Work out the best day for the event. */
    let mut best_days = ord_freq
        .iter()
        .filter(|x| x.1 == ord_freq[0].1)
        .map(|x| x.0 + 1)
        .collect::<Vec<usize>>();
    best_days.sort();

    /* Show the best days. */
    println!(
        "{}",
        best_days
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}
