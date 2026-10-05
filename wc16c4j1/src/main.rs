/*
 * WC '16 Contest 4 J1 - Anyone Can Be Anything
 * https://dmoj.ca/problem/wc16c4j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of animals. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();

    /* Read the number of animals who want jobs in each category */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut job_openings = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Read the job preferences. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let animal_preferences = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect::<Vec<usize>>();

    /* Allocate the animals to the jobs they want if they are available. */
    let mut allocated_right = 0;
    for pref_idx in animal_preferences.into_iter() {
        if job_openings[pref_idx] > 0 {
            job_openings[pref_idx] -= 1;
            allocated_right += 1;
        }
    }

    /* How many animals got the job they wanted? */
    println!("{}", allocated_right);
}
