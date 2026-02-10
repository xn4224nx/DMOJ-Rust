/*
 * 3n+1
 * https://dmoj.ca/problem/collatz
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!(
        "{}",
        collatz_decent(buffer.trim_end().parse::<usize>().unwrap())
    );
}

/// How many many changes does it take to reach one.
fn collatz_decent(inital_value: usize) -> usize {
    let mut val = inital_value;
    let mut steps = 0;

    while val > 1 {
        steps += 1;

        /* If the value is even, the next term is one half of it. If the value
         * is odd, the next term is three times the value plus one. */
        if val % 2 == 0 {
            val /= 2;
        } else {
            val = 3 * val + 1;
        }
    }
    return steps;
}
