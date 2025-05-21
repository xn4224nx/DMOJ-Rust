/*
 * CCC '20 J2 - Epidemiology
 * https://dmoj.ca/problem/ccc20j2
 */

fn main() {
    let mut day = 0;
    let limit = read_number();
    let mut prev_infected = 0;
    let mut curr_infected = read_number();
    let growth = read_number();

    /* Infect people until the limit is breached. */
    while prev_infected + curr_infected <= limit {
        prev_infected += curr_infected;
        curr_infected *= growth;
        day += 1;
    }

    /* Return the number of days it took to breach the limit. */
    println!("{}", day);
}

fn read_number() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse::<u32>().unwrap();
}
