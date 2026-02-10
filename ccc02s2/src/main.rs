/*
 * CCC '02 S2 - Fraction Action
 * https://dmoj.ca/problem/ccc02s2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the fraction. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse the fraction. */
    let frac = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("{}", simplify_frac(frac[0], frac[1]));
}

/// Find the simplest form of a fraction.
fn simplify_frac(numerator: usize, denominator: usize) -> String {
    /* Catch special cases. */
    if numerator == 0 {
        return numerator.to_string();
    }

    /* Find the highest common factor between them. */
    let hcf = gcd(numerator, denominator);

    /* Simplify the fraction. */
    let simp_numr = numerator / hcf;
    let simp_deno = denominator / hcf;

    /* Is the number still a fraction. */
    if simp_deno == 1 {
        return simp_numr.to_string();
    }

    /* Convert top heavy fractions. */
    if simp_numr > simp_deno {
        return format!(
            "{} {}/{}",
            simp_numr / simp_deno,
            simp_numr % simp_deno,
            simp_deno
        );
    } else {
        return format!("{}/{}", simp_numr, simp_deno);
    }
}

fn gcd(a_val: usize, b_val: usize) -> usize {
    let mut a = a_val;
    let mut b = b_val;

    if a == b {
        return a;
    }

    // Swap a with b, if b is greater than a
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}
