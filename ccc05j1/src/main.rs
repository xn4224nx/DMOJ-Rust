/*
 * CCC '05 J1 - The Cell Sell
 * https://dmoj.ca/problem/ccc05j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of minutes. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse the minutes. */
    let minutes = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* How much would plan A cost? */
    let plan_a = minutes[0].saturating_sub(100) * 25 + 15 * minutes[1] + 20 * minutes[2];

    /* How much would plan B cost? */
    let plan_b = minutes[0].saturating_sub(250) * 45 + 35 * minutes[1] + 25 * minutes[2];

    /* Show the prices. */
    println!("Plan A costs {}.{:02}", plan_a / 100, plan_a % 100);
    println!("Plan B costs {}.{:02}", plan_b / 100, plan_b % 100);

    /* What plan is cheaper? */
    println!(
        "{}",
        if plan_a < plan_b {
            "Plan A is cheapest."
        } else if plan_a > plan_b {
            "Plan B is cheapest."
        } else {
            "Plan A and B are the same price."
        }
    );
}
