/*
 * CCC '10 S1 - Computer Purchase
 * https://dmoj.ca/problem/ccc10s1
 */

fn main() {
    let mut buffer = String::new();

    /* How many computers are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_comps = buffer.trim_end().parse::<usize>().unwrap();

    let mut comps = Vec::with_capacity(num_comps);

    /* Read the computer infomation. */
    for _ in 0..num_comps {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let data = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        comps.push((
            usize::MAX
                - (2 * data[1].parse::<usize>().unwrap()
                    + 3 * data[2].parse::<usize>().unwrap()
                    + data[3].parse::<usize>().unwrap()),
            data[0].clone(),
        ));
    }

    /* Handle the case of only one computer. */
    if comps.len() == 1 {
        println!("{}", comps[0].1);
    } else if comps.len() > 1 {
        comps.sort();
        println!("{}\n{}", comps[0].1, comps[1].1);
    }
}
