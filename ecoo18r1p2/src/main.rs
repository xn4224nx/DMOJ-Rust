/*
 * ECOO '18 R1 P2 - Rue's Rings
 * https://dmoj.ca/problem/ecoo18r1p2
 */

fn main() {
    let mut buffer = String::new();

    for _ in 0..10 {
        buffer.clear();

        /* Read the number of routes. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let num_rounds = buffer.trim().parse::<usize>().unwrap();

        /* Read the roundabout data. */
        let mut rout_ids: Vec<usize> = Vec::with_capacity(num_rounds);
        let mut rout_data: Vec<Vec<usize>> = Vec::with_capacity(num_rounds);

        for _ in 0..num_rounds {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
            let data: Vec<usize> = buffer
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            rout_ids.push(data[0]);
            rout_data.push(data[2..].iter().cloned().collect());
        }

        /* Find the smallest roundabout in the data */
        let min_diam: usize = *rout_data.iter().flatten().min().unwrap();

        /* Find the routes that contain a roundabout of this size. */
        let mut concer_routes: Vec<usize> = (0..rout_data.len())
            .filter(|x| rout_data[*x].contains(&min_diam))
            .map(|x| rout_ids[x])
            .collect();
        concer_routes.sort();
        let out_routes = concer_routes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("{} {{{}}}", min_diam, out_routes);
    }
}
