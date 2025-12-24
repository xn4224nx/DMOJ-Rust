/*
 * Waterloo 2017 Winter A - Vera and Outfits
 * https://dmoj.ca/problem/waterloo2017wa
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of tops and pants. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num = buffer.trim_end().parse::<usize>().unwrap();

    /* Show the number of outfit combinations. */
    println!("{}", num * num.saturating_sub(1));
}
