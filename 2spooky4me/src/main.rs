/*
 * 2spooky4me - PEG Test - Halloween 2014
 * https://dmoj.ca/problem/2spooky4me
 */

fn main() {
    let mut buffer = String::new();

    /* Read the senario constants. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let sec_data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    /* Keep a record of combined house spookiness. */
    let mut house_spook: Vec<(i64, i64)> = vec![(sec_data[1] + 1, 0)];

    /* Read the details of each decoration. */
    for _ in 0..sec_data[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let decr = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        /* The spookiness increases and the decoration starts. */
        house_spook.push((decr[0], decr[2]));

        /* The decoration ends and the spookines decreases. */
        house_spook.push((decr[1] + 1, -decr[2]));
    }

    house_spook.sort_by(|x, y| x.0.cmp(&y.0));

    /* Calculate the number of houses that can be visited. */
    let mut visited_houses = sec_data[1];
    let mut current = 0;

    for idx in 0..(house_spook.len() - 1) as usize {
        current += house_spook[idx].1;

        /* Exclude houses that are too spooky to visit. */
        if current >= sec_data[2] {
            visited_houses -= house_spook[idx + 1].0 - house_spook[idx].0;
        }
    }
    println!("{}", visited_houses);
}
