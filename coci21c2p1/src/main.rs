/*
 * COCI '21 Contest 2 #1 Kaučuk
 * https://dmoj.ca/problem/coci21c2p1
 */

fn main() {
    let mut buffer = String::new();
    let mut sec_count = vec![0, 0, 0];

    /* How many headings will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_headings = buffer.trim_end().parse::<usize>().unwrap();

    /* Read each heading in turn and print it. */
    for _ in 0..num_headings {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let (sec_type, title) = buffer.trim_end().split_once(" ").unwrap();

        if sec_type == "section" {
            sec_count[0] += 1;
            sec_count[1] = 0;
            sec_count[2] = 0;
            println!("{} {}", sec_count[0], title);
        } else if sec_type == "subsection" {
            sec_count[1] += 1;
            sec_count[2] = 0;
            println!("{}.{} {}", sec_count[0], sec_count[1], title);
        } else {
            sec_count[2] += 1;
            println!(
                "{}.{}.{} {}",
                sec_count[0], sec_count[1], sec_count[2], title
            );
        }
    }
}
