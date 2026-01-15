/*
 * CCC '14 S3 - The Geneva Confection
 * https://dmoj.ca/problem/ccc14s3
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of cases. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cases = buffer.trim_end().parse::<usize>().unwrap();

    for _ in 0..num_cases {
        buffer.clear();

        /* Read the number of trollies. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let num_troll = buffer.trim_end().parse::<usize>().unwrap();

        let mut trollies = Vec::with_capacity(num_troll);

        /* Read the trolly order. */
        for _ in 0..num_troll {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
            trollies.push(buffer.trim_end().parse::<usize>().unwrap());
        }
        println!("{}", if does_order_work(&trollies) { "Y" } else { "N" })
    }
}

fn does_order_work(troll_state: &Vec<usize>) -> bool {
    let mut branch = Vec::new();
    let mut lake_need = 1;

    /* The trollies decend to the lake or the branch */
    for trol in troll_state.iter().rev() {
        /* Check if the branch can be used. */
        loop {
            if !branch.is_empty() && branch[branch.len() - 1] == lake_need {
                lake_need = branch.pop().unwrap() + 1;
            } else {
                break;
            }
        }

        /* Does the trolly go into the lake or the branch? */
        if *trol == lake_need {
            lake_need += 1;
        } else {
            branch.push(*trol)
        }
    }

    /* Do all the trollies in the branch go into the lake? */
    while !branch.is_empty() {
        if branch.pop().unwrap() != lake_need {
            return false;
        } else {
            lake_need += 1;
        }
    }
    return true;
}
