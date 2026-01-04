/*
 * CCC '19 S1 - Flipper
 * https://dmoj.ca/problem/ccc19s1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the transformations. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Read the number of each type of flip. */
    let mut num_h = 0;
    let mut num_v = 0;
    for flip in buffer.chars() {
        match flip {
            'H' => num_h += 1,
            'V' => num_v += 1,
            _ => {}
        }
    }

    /* The flips of the same type cancel each other out. */
    num_h %= 2;
    num_v %= 2;

    print!(
        "{}",
        /* Neither axis is flipped. */
        if num_h == 0 && num_v == 0 {
            "1 2\n3 4\n"

        /* Both axis are flipped */
        } else if num_h == 1 && num_v == 1 {
            "4 3\n2 1\n"

        /* Only the vertical is flipped. */
        } else if num_v == 1 {
            "2 1\n4 3\n"

        /* Only the horizontal is flipped. */
        } else {
            "3 4\n1 2\n"
        }
    );
}
