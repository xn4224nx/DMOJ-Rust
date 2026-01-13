/*
 * CCC '05 J2 - RSA Numbers
 * https://dmoj.ca/problem/ccc05j2
 */

const RSA_DIVISORS: usize = 4;

fn main() {
    let mut buffer = String::new();

    /* Read the two sides of the range. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let lower_lim = buffer.trim_end().parse::<usize>().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let upper_lim = buffer.trim_end().parse::<usize>().unwrap();

    println!(
        "The number of RSA numbers between {} and {} is {}",
        lower_lim,
        upper_lim,
        (lower_lim..=upper_lim)
            .into_iter()
            .filter(|x| is_rsa_num(*x))
            .count()
    );
}

/// Determine of a number is a RSA number
fn is_rsa_num(num: usize) -> bool {
    let mut total_divisors = 0;

    for div_num in 1..=num {
        if total_divisors > RSA_DIVISORS {
            return false;
        } else if num % div_num == 0 {
            total_divisors += 1;
        }
    }
    return total_divisors == RSA_DIVISORS;
}
