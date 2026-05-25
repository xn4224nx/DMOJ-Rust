/*
 * CCC '13 S5 - Factor Solitaire
 * https://dmoj.ca/problem/ccc13s5
 */

fn main() {
    let mut buffer = String::new();

    /* Read the target value. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    println!(
        "{}",
        factor_solitaire_cost(buffer.trim_end().parse::<usize>().unwrap())
    );
}

fn factor_solitaire_cost(target: usize) -> usize {
    let mut cur_targ = target;
    let mut cost = 0;

    while cur_targ > 1 {
        let mut fact = 2;

        /* Find the highest possible factor. */
        while fact <= cur_targ.isqrt() + 1 && cur_targ % fact != 0 {
            fact += 1;
        }

        /* Ensure the factor is not a prime or one. */
        if fact < cur_targ && cur_targ % fact == 0 {
            let div = cur_targ / fact;
            cur_targ -= div;
            cost += cur_targ / div;

        /* Handle the factor being a prime. */
        } else {
            cur_targ -= 1;
            cost += cur_targ;
        }
    }
    return cost;
}
