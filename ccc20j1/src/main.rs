/*
 * CCC '20 J1 - Dog Treats
 * https://dmoj.ca/problem/ccc20j1
 */

fn main() {
    let num_treats = 3;
    let mut buffer = String::new();
    let mut treats_rec: Vec<usize> = Vec::with_capacity(num_treats);

    /* Read the number of each treat the dog recieved. */
    for _ in 0..num_treats {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        treats_rec.push(
            buffer
                .trim_end()
                .parse::<usize>()
                .expect("Treat amount could not be read!"),
        );
    }

    let happy_score = treats_rec
        .into_iter()
        .enumerate()
        .map(|(idx, scr)| (idx + 1) * scr)
        .sum::<usize>();

    if happy_score >= 10 {
        println!("happy");
    } else {
        println!("sad")
    }
}
