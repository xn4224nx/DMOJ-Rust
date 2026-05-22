/*
 * CPC '21 Contest 1 P0 - AQT and Alphabet
 * https://dmoj.ca/problem/cpc21c1p0
 */

fn main() {
    let mut buffer = String::new();

    /* Read the input letters. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Iterate over them and find the first missing letter of the alphabet */
    'alpha_check: for alpha in vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
    .into_iter()
    {
        for letter in buffer.chars() {
            if alpha == letter {
                continue 'alpha_check;
            }
        }

        /* The first letter that cannot be found is the solution. */
        println!("{}", alpha);
        return;
    }
    panic!("No letter match has been found!");
}
