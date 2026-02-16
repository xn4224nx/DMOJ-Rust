/*
 * CCC '24 J2 - Dusa And The Yobis
 * https://dmoj.ca/problem/ccc24j2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the initial size. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut curr_weight = buffer.trim_end().parse::<usize>().unwrap();

    /* Acumulate mass until nothing more can be digested. */
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let new_weight = buffer.trim_end().parse::<usize>().unwrap();

        /* If the new weight is too big stop and show the final weight. */
        if new_weight < curr_weight {
            curr_weight += new_weight;
        } else {
            println!("{}", curr_weight);
            return;
        }
    }
}
