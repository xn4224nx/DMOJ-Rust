/*
 * COCI '06 Regional #3 Firefly
 * https://dmoj.ca/problem/crci06p3
 */

fn main() {
    let mut buffer = String::new();

    /* Read the cage dimensions. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let cave_dims = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut mites_obs = Vec::with_capacity(1 + cave_dims[0] / 2);
    let mut tites_obs = Vec::with_capacity(1 + cave_dims[0] / 2);

    /* Read the obstacle sizes. */
    for obs_idx in 0..cave_dims[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mag = buffer.trim().parse::<usize>().unwrap();

        /* In the cave is this a stalactite or a stalagmite? */
        if obs_idx % 2 == 0 {
            mites_obs.push(mag);
        } else {
            tites_obs.push(mag);
        }
    }

    /* Location is irrelevant at this point. */
    mites_obs.sort();
    tites_obs.sort();

    /* Determine how many obsticles a firefly would hit at each height. */
    let mut optimal_heights = 1;
    let mut min_collisions = cave_dims[0];

    for height in 1..=cave_dims[1] {
        let collisions = cave_dims[0]
            - mites_obs.partition_point(|&x| x < height)
            - tites_obs.partition_point(|&x| x < cave_dims[1] + 1 - height);

        /* Has a better path been found? */
        if collisions < min_collisions {
            min_collisions = collisions;
            optimal_heights = 1;

        /* Has an equivilent path been found? */
        } else if collisions == min_collisions {
            optimal_heights += 1;
        }
    }
    println!("{} {}", min_collisions, optimal_heights);
}
