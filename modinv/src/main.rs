/*
 * Modular Multiplicative Inverse
 * https://dmoj.ca/problem/modinv
 */

fn main() {
    let mut buffer = String::new();

    /* Read the variables N and M. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    println!("{}", find_mod_inverse(data[0], data[1]));
}

/// This assumes that N and M are co primes.
fn find_mod_inverse(n_var: i128, m_var: i128) -> i128 {
    let mut m_0: i128 = m_var;
    let mut n_0: i128 = n_var;
    let mut y_0: i128 = 0;
    let mut x_0: i128 = 1;
    let mut tmp: i128;

    /* Simple case. */
    if m_0 <= 1 {
        return 0;
    }

    while n_0 > 1 {
        /* If this triggers it means that gcd(N, M) != 1 and they are not coprime. */
        if m_0 == 0 {
            return 0;
        }

        let quotient = n_0 / m_0;
        tmp = m_0;

        /* M is  now the remainder. */
        m_0 = n_0 % m_0;
        n_0 = tmp;
        tmp = y_0;

        /* Update X and Y. */
        y_0 = x_0 - quotient * y_0;
        x_0 = tmp;
    }

    /* Ensure that X is positive. */
    if x_0 < 0 {
        x_0 += m_var;
    }
    return x_0;
}
