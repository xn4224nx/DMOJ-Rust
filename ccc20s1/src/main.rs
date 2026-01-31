/*
 * CCC '20 S1 - Surmising a Sprinter's Speed
 * https://dmoj.ca/problem/ccc20s1
 */

fn main() {
    let mut buffer = String::new();
    let mut max_speed: f64 = 0.0;

    /* How many observations are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_obs = buffer.trim_end().parse::<usize>().unwrap();
    let mut rr_obs: Vec<(f64, f64)> = Vec::with_capacity(num_obs);

    for _ in 0..num_obs {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Read the time elapsed and position. */
        let data = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        rr_obs.push((data[0], data[1]));
    }

    /* Sort the events in time. */
    rr_obs.sort_by(|x, y| x.partial_cmp(y).unwrap());

    /* Compare each point to the next one in time and find the maximum speed. */
    for obs_idx in 1..num_obs {
        let tmp_speed = ((rr_obs[obs_idx].1 - rr_obs[obs_idx - 1].1)
            / (rr_obs[obs_idx].0 - rr_obs[obs_idx - 1].0))
            .abs();

        if tmp_speed > max_speed {
            max_speed = tmp_speed;
        }
    }
    println!("{:?}", max_speed);
}
