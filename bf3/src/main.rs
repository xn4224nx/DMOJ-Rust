/*
 * Next Prime
 * https://dmoj.ca/problem/bf3
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut value = buffer.trim().parse::<u128>().unwrap();

    loop {
        if is_prime(value) {
            println!("{}", value);
            return;
        }
        value += 1;
    }
}

/// Is the number prime?
fn is_prime(num: u128) -> bool {
    /* Handle special cases. */
    if num < 2 {
        return false;
    } else if num == 2 {
        return true;
    }

    /* Check numbers for factor. */
    for fac in 2..(num.isqrt() + 1) {
        if num % fac == 0 {
            return false;
        }
    }
    return true;
}
