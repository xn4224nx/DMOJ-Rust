/*
 * CCC '14 S1 - Party Invitation
 * https://dmoj.ca/problem/ccc14s1
 */

fn main() {
    let mut buffer = String::new();

    /* How many people are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_people = buffer.trim_end().parse::<usize>().unwrap();

    /* How many removal rounds are there? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_rounds = buffer.trim_end().parse::<usize>().unwrap();

    /* What are the removal rounds? */
    let mut rounds: Vec<usize> = Vec::with_capacity(num_rounds);
    for _ in 0..num_rounds {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        rounds.push(buffer.trim().parse::<usize>().unwrap());
    }

    /* Keep a record of people not eliminated. */
    let mut rem_people = vec![true; num_people];

    /* Simulate the person removal due to the rounds. */
    for rnd in rounds.into_iter() {
        let mut sim_idx = 0;

        for plp_idx in 0..num_people {
            /* Is this person still there? */
            if !rem_people[plp_idx] {
                continue;
            }

            /* Is this person eliminated? */
            if (sim_idx + 1) % rnd == 0 {
                rem_people[plp_idx] = false;
            }

            /* What is the next simulated index? */
            sim_idx += 1;
        }
    }

    /* Show the people that remain? */
    for plp_idx in 0..num_people {
        if rem_people[plp_idx] {
            println!("{}", plp_idx + 1);
        }
    }
}
