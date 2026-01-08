/*
 * CCC '13 J4 - Time on task
 * https://dmoj.ca/problem/ccc13j4
 */

fn main() {
    let mut buffer = String::new();

    /* What is the time limit. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let time_limit = buffer.trim_end().parse::<usize>().unwrap();

    /* How many chores are there? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_chores = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the time the chores take to complete. */
    let mut chores = Vec::with_capacity(num_chores);
    for _ in 0..num_chores {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        chores.push(buffer.trim_end().parse::<usize>().unwrap());
    }

    chores.sort();

    /* Determine the maximum number of chores that can be completed. */
    let mut completed_tasks = 0;
    let mut time_used = 0;

    for chor_idx in 0..num_chores {
        if time_used + chores[chor_idx] <= time_limit {
            completed_tasks += 1;
            time_used += chores[chor_idx];
        } else {
            break;
        }
    }
    println!("{}", completed_tasks);
}
