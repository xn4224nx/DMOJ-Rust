/*
 * CCC '26 S2 - Beams of Light
 * https://dmoj.ca/problem/ccc26s2
 */

fn main() {
    let (num_parks, num_lights, num_qs) = read_metadata();
    let light_cover = read_illumin(num_parks, num_lights);
    answer_questions(num_qs, &light_cover);
}

/// Read the three problem metadata values from STDIN.
fn read_metadata() -> (usize, usize, usize) {
    let mut buffer = String::new();
    for _ in 0..3 {
        std::io::stdin().read_line(&mut buffer).unwrap();
    }
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    return (data[0], data[1], data[2]);
}

/// Read the light coverage from STDIN and determined the illumination in car park.
fn read_illumin(num_parks: usize, num_lights: usize) -> Vec<usize> {
    let mut coverage: Vec<i32> = vec![0; num_parks + 2];
    let mut buffer = String::new();

    /* Read the bulb location and radius. */
    for _ in 0..num_lights {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let bulb = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* How far does this bulb illuminate? */
        let lower_reach = bulb[0].saturating_sub(bulb[1]);
        let upper_reach = std::cmp::min(bulb[0] + bulb[1], num_parks) + 1;

        /* Mark where the ilumination begins and ends. */
        coverage[lower_reach] += 1;
        coverage[upper_reach] -= 1;
    }

    /* Fill in the gaps in the coverage */
    for idx in 1..=num_parks {
        coverage[idx] += coverage[idx - 1]
    }

    return coverage.into_iter().map(|x| x as usize).collect();
}

/// Read queries from STDIN and answer them.
fn answer_questions(num_qs: usize, coverage: &Vec<usize>) {
    let mut buffer = String::new();
    for _ in 0..num_qs {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let p_idx = buffer.trim_end().parse::<usize>().unwrap();
        println!("{}", if coverage[p_idx] > 0 { "Y" } else { "N" });
    }
}
