/*
 * COCI '20 Contest 1 #1 Patkice
 * https://dmoj.ca/problem/coci20c1p1
 */

use std::collections::HashMap;

struct SeaVoyage {
    seamap: HashMap<(usize, usize), char>,
    start: (usize, usize),
}

impl SeaVoyage {
    fn new() -> Self {
        let mut buffer = String::new();

        /* Read the map dimensions */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let dims = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut seamap: HashMap<(usize, usize), char> = HashMap::new();
        let mut start = (0, 0);

        /* Read the map. */
        for row_idx in 0..dims[0] {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();

            /* Parse each row element by element, ignoring calm sea. */
            for (col_idx, ele) in buffer.trim().chars().enumerate() {
                if ele != '.' {
                    seamap.insert((row_idx, col_idx), ele);
                }

                /* Locate the start point.  */
                if ele == 'o' {
                    start = (row_idx, col_idx);
                }
            }
        }
        return SeaVoyage { seamap, start };
    }

    /// Find the shortest route from the start point to the end point and if it
    /// exists print the initial direction it starts in.
    fn traverse(&self) {
        /* There will be a maximum of four paths accross the sea. */
        let directs = vec!['N', 'E', 'S', 'W'];
        let mut path_viable = vec![true; 4];
        let mut curr_pnts = vec![
            (self.start.0.overflowing_sub(1).0, self.start.1),
            (self.start.0, self.start.1 + 1),
            (self.start.0 + 1, self.start.1),
            (self.start.0, self.start.1.overflowing_sub(1).0),
        ];

        /* Keep a record of inital directions of found solutions. */
        let mut solu_directs: Vec<char> = Vec::new();

        /* Keep following the paths until the end point is reached or we run
         * out of paths to follow. */
        while solu_directs.is_empty() && path_viable.contains(&true) {
            for p_idx in 0..4 {
                let pnt = curr_pnts[p_idx];

                if !path_viable[p_idx] {
                    continue;
                }

                /* Ensure the point it is on is viable. */
                if !self.seamap.contains_key(&pnt) {
                    path_viable[p_idx] = false;
                    continue;
                }

                /* Move to the next point based on the current direction */
                match self.seamap.get(&pnt).unwrap() {
                    '^' => curr_pnts[p_idx] = (pnt.0.overflowing_sub(1).0, pnt.1),
                    '>' => curr_pnts[p_idx] = (pnt.0, pnt.1 + 1),
                    'v' => curr_pnts[p_idx] = (pnt.0 + 1, pnt.1),
                    '<' => curr_pnts[p_idx] = (pnt.0, pnt.1.overflowing_sub(1).0),
                    'o' => path_viable[p_idx] = false,
                    'x' => solu_directs.push(directs[p_idx]),
                    _ => panic!("Unknown point!"),
                }
            }
        }

        /* No path to the result. */
        if !path_viable.contains(&true) {
            println!(":(");

        /* We found away accross the sea. */
        } else if !solu_directs.is_empty() {
            solu_directs.sort();
            println!(":)\n{}", solu_directs[0]);

        /* This option should never be possible! */
        } else {
            panic!("Impossible traversal result reached!");
        }
    }
}

fn main() {
    SeaVoyage::new().traverse();
}
