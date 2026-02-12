/*
 * CCC '96 S1 - Deficient, Perfect, and Abundant
 * https://dmoj.ca/problem/ccc96s1
 */

fn main() {
    let mut buffer = String::new();

    /* How many test cases are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cases = buffer.trim_end().parse::<usize>().unwrap();

    for _ in 0..num_cases {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        println!(
            "{}",
            num_indent(buffer.trim_end().parse::<usize>().unwrap())
        );
    }
}

fn num_indent(number: usize) -> String {
    let mut factor_sum = 1;

    /* Sum every factor of the number. */
    for pos_factor in 2..number {
        if number % pos_factor == 0 {
            factor_sum += pos_factor;
        }
    }

    /* What type of number is it? */
    return if factor_sum < number {
        format!("{} is a deficient number.", number)
    } else if factor_sum > number {
        format!("{} is an abundant number.", number)
    } else {
        format!("{} is a perfect number.", number)
    };
}
