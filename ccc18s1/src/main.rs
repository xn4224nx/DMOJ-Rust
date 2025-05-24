/*
 * CCC '18 S1 - Voronoi Villages
 * https://dmoj.ca/problem/ccc18s1
 */

fn main() {
    let mut village_pos = Vec::new();
    let mut buffer = String::new();

    /* Read the number of lines to be compressed. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_lines = buffer.trim().parse::<usize>().unwrap();

    /* Read the village positions. */
    for _ in 0..num_lines {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        village_pos.push(buffer.trim().parse::<i32>().unwrap());
    }

    /* Sort the positions */
    village_pos.sort();

    /* Iterate over the positions and calculate the smallest neighbourhood.  */
    let mut min_size = f64::MAX;
    for idx in 1..village_pos.len() - 1 {
        let lower_lim = (village_pos[idx] - village_pos[idx - 1]) as f64 / 2.0;
        let upper_lim = (village_pos[idx + 1] - village_pos[idx]) as f64 / 2.0;
        let neigh_size = upper_lim + lower_lim;

        if neigh_size < min_size {
            min_size = neigh_size;
        }
    }
    println!("{:.1}", min_size)
}
