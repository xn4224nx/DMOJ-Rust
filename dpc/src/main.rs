/*
 * Educational DP Contest AtCoder C - Vacation
 * https://dmoj.ca/problem/dpc
 */

fn main() {
    let mut buffer = String::new();
    let num_options = 3;

    /* How many days will their be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_days = buffer.trim_end().parse::<usize>().unwrap();

    /* Keep a record of the running total of happiness. */
    let mut curr_happy = vec![0; num_options];

    /* Process the happiness day by day. */
    for _ in 0..num_days {
        buffer.clear();

        /* Read and parse the day's options. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let happy_vals = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        assert_eq!(num_options, happy_vals.len());

        /* Consider the happiness gained by picking the best option. */
        curr_happy = (0..num_options)
            .map(|x| {
                happy_vals[x]
                    + (0..num_options)
                        .filter(|y| *y != x)
                        .map(|y| curr_happy[y])
                        .max()
                        .unwrap()
            })
            .collect::<Vec<usize>>();
    }

    /* Show the maximum possible happiness. */
    println!("{}", curr_happy.into_iter().max().unwrap());
}
