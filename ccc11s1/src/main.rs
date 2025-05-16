/*
 * CCC '11 S1 - English or French?
 * https://dmoj.ca/problem/ccc11s1
 */

fn main() {
    let mut s_cnts = 0;
    let mut t_cnts = 0;

    let mut input = String::new();

    /* How many sentences should be expected? */
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();
    let num_sentc = input.parse::<usize>().unwrap();
    input.clear();

    /* Count the number of t and s chars in the coming sentences. */
    for _ in 0..num_sentc {
        std::io::stdin().read_line(&mut input).unwrap();
        for s_char in input.chars() {
            match s_char {
                'S' | 's' => s_cnts += 1,
                'T' | 't' => t_cnts += 1,
                _ => {}
            }
        }
        input.clear();
    }

    /* Use the counts to guess at the language the text is. */
    if t_cnts > s_cnts {
        println!("English");
    } else {
        println!("French");
    }
}
