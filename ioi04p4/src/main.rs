/*
 * IOI '04 P4 - Phidias
 * https://dmoj.ca/problem/ioi04p4
 */

fn main() {
    let mut cuts = vec![vec![0; 602]; 602];
    let mut waste = vec![vec![0; 602]; 602];

    let mut buffer = String::new();

    /* Read the slab dimensions. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dim: Vec<usize> = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    /* Read the number of plate sizes. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_plates = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the actual plate sizes. */
    for _ in 1..=num_plates {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let pl_size: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        cuts[pl_size[0]][pl_size[1]] = 1;
    }

    /* Determine the remaining stone. */
    for x in 1..=dim[0] {
        for y in 1..=dim[1] {
            if cuts[x][y] == 1 {
                continue;
            }

            waste[x][y] = x * y;

            for x1 in 1..=x / 2 {
                waste[x][y] = std::cmp::min(waste[x][y], waste[x1][y] + waste[x - x1][y]);
            }

            for y1 in 1..=y / 2 {
                waste[x][y] = std::cmp::min(waste[x][y], waste[x][y1] + waste[x][y - y1]);
            }
        }
    }

    /* Output the minimum wasted slab area. */
    println!("{}", waste[dim[0]][dim[1]]);
}
