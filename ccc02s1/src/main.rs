/*
 * CCC '02 S1 - The Students' Council Breakfast
 * https://dmoj.ca/problem/ccc02s1
 */

fn main() {
    let mut buffer = String::new();
    let mut total_valid_combinations = 0;
    let mut min_num_tickets = usize::MAX;

    /* What is the cost of pink tickets? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let cost_pink = buffer.trim_end().parse::<usize>().unwrap();

    /* What is the cost of green tickets? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let cost_green = buffer.trim_end().parse::<usize>().unwrap();

    /* What is the cost of red tickets? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let cost_red = buffer.trim_end().parse::<usize>().unwrap();

    /* What is the cost of orange tickets? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let cost_orange = buffer.trim_end().parse::<usize>().unwrap();

    /* How much money needs to be raised? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let total_money_needed = buffer.trim_end().parse::<usize>().unwrap();

    /* Determined the maximum number of each ticket that could be sold. */
    let max_pink = (total_money_needed / cost_pink) + 1;
    let max_green = (total_money_needed / cost_green) + 1;
    let max_red = (total_money_needed / cost_red) + 1;
    let max_orange = (total_money_needed / cost_orange) + 1;

    /* Try every combination of tickets. */
    for num_pink in 0..max_pink {
        for num_green in 0..max_green {
            for num_red in 0..max_red {
                for num_orange in 0..max_orange {
                    let money_raised = cost_pink * num_pink
                        + cost_green * num_green
                        + cost_red * num_red
                        + cost_orange * num_orange;

                    /* Does this combination raise the right amount? */
                    if money_raised == total_money_needed {
                        let total_tickets = num_pink + num_green + num_red + num_orange;
                        println!(
                            "# of PINK is {} # of GREEN is {} # of RED is {} # of ORANGE is {}",
                            num_pink, num_green, num_red, num_orange
                        );
                        total_valid_combinations += 1;

                        /* Is this combination the lowest in terms of number of tickets? */
                        if total_tickets < min_num_tickets {
                            min_num_tickets = total_tickets;
                        }
                    }
                }
            }
        }
    }

    /* Show the final stats */
    println!(
        "Total combinations is {}.\nMinimum number of tickets to print is {}.",
        total_valid_combinations, min_num_tickets
    );
}
