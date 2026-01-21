/*
 * CCC '22 S1 - Good Fours and Good Fives
 * https://dmoj.ca/problem/ccc22s1
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    println!(
        "{}",
        sum_comb(4, 5, buffer.trim_end().parse::<usize>().unwrap())
    );
}

/// Count the number of ways the two values can be used to create the target.
pub fn sum_comb(val_0: usize, val_1: usize, target: usize) -> usize {
    let mut total_comb = 0;

    for fact in 0..=(target / val_0) {
        if target >= fact * val_0 && (target - (fact * val_0)) % val_1 == 0 {
            total_comb += 1;
        }
    }
    return total_comb;
}
